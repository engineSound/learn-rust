// Front end shared by learn-nodejs and learn-rust. Both servers expose the same API:
//   GET  /api/info                      -> { track, title, accent, tutor: { enabled, reason } }
//   GET  /api/lessons                   -> [{ slug, id, title, minutes, concepts, kind, for, stats }]
//   GET  /api/lessons/:slug             -> { slug, meta, body, file, code, saved, stats }
//   PUT  /api/lessons/:slug/code        (text body) -> { saved: true }
//   DELETE /api/lessons/:slug/code      -> { code, stats, next }  (back to the starter)
//   POST /api/lessons/:slug/run         (text body) -> run result + { stats, next }
//   POST /api/lessons/:slug/feedback    (easy|right|hard) -> { stats, next }
//   GET  /api/profile                   -> { profile, markdown, next }
//   POST /api/lessons/:slug/hint        ({ code, question }) -> { hint }
//   POST /api/lessons/:slug/drill       -> { slug, title, targets, stats, next }  (a new, verified drill)
// `next` is the adaptive next step (SHARED-FORMAT.md, section 5) plus its `slug`.
// `stats` is the learner's history for that lesson (see SHARED-FORMAT.md).
(function () {
  var $ = function (id) { return document.getElementById(id); };
  var select = $('lesson-select'), code = $('code'), gutter = $('gutter');
  var runBtn = $('run-btn'), resetBtn = $('reset-btn'), profileBtn = $('profile-btn'), results = $('results'), saveState = $('save-state');
  var current = null, saveTimer = null, lessonList = [], tutor = { enabled: false, reason: null };

  function api(method, path, body) {
    return fetch(path, { method: method, body: body, headers: body != null ? { 'Content-Type': 'text/plain; charset=utf-8' } : {} })
      .then(function (r) {
        return r.json().then(function (data) {
          if (!r.ok) throw new Error(data.error || ('HTTP ' + r.status));
          return data;
        });
      });
  }
  function esc(s) {
    return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }
  function ago(ts) {
    var mins = Math.round((Date.now() - Date.parse(ts)) / 60000);
    if (mins < 2) return 'just now';
    if (mins < 60) return mins + ' min ago';
    var hours = Math.round(mins / 60);
    if (hours < 36) return hours + ' h ago';
    return Math.round(hours / 24) + ' days ago';
  }
  function statusText(st) {
    if (!st || !st.runs) return 'New';
    if (st.due_now) return '↻ Review due';
    if (st.first_pass_run) return '✓ Solved on run ' + st.first_pass_run;
    var runs = st.runs + (st.runs === 1 ? ' run' : ' runs');
    return st.total ? runs + ', best ' + st.best_passed + '/' + st.total : runs + ', not compiling yet';
  }
  function optionLabel(l) {
    var st = l.stats || {};
    var kind = l.kind === 'drill' ? '↳ Drill: ' : l.kind === 'stretch' ? '↳ Stretch: ' : '';
    return (st.due_now ? '↻ ' : st.first_pass_run ? '✓ ' : '') + kind + l.title + ' · ' + (l.minutes || '?') + ' min';
  }
  function updateStats(stats) {
    current.stats = stats;
    $('lesson-status').textContent = statusText(stats);
    $('lesson-status').className = 'chip status' + (stats && stats.due_now ? ' due' : stats && stats.first_pass_run ? ' done' : '');
    lessonList.forEach(function (l) { if (l.slug === current.slug) l.stats = stats; });
    var opt = select.querySelector('option[value="' + current.slug + '"]');
    var item = lessonList.filter(function (l) { return l.slug === current.slug; })[0];
    if (opt && item) opt.textContent = optionLabel(item);
  }

  function remember(slug) { try { localStorage.setItem('learn.lesson', slug); } catch (e) {} }
  function recall() { try { return localStorage.getItem('learn.lesson'); } catch (e) { return null; } }

  // Editor: line numbers, Tab indents, Ctrl/Cmd+Enter runs.
  function updateGutter() {
    var n = code.value.split('\n').length, s = '';
    for (var i = 1; i <= n; i++) s += i + '\n';
    gutter.textContent = s;
    gutter.scrollTop = code.scrollTop;
  }
  code.addEventListener('scroll', function () { gutter.scrollTop = code.scrollTop; });
  code.addEventListener('input', function () { updateGutter(); scheduleSave(); });
  code.addEventListener('keydown', function (e) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') { e.preventDefault(); run(); return; }
    if (e.key === 'Tab' && !e.shiftKey && !e.ctrlKey && !e.metaKey) {
      e.preventDefault();
      var indent = current && /\.rs$/.test(current.file) ? '    ' : '  ';
      var s = code.selectionStart, end = code.selectionEnd;
      code.value = code.value.slice(0, s) + indent + code.value.slice(end);
      code.selectionStart = code.selectionEnd = s + indent.length;
      updateGutter(); scheduleSave();
    }
  });
  document.addEventListener('keydown', function (e) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter' && e.target !== code) { e.preventDefault(); run(); }
  });

  function scheduleSave() {
    if (!current) return;
    saveState.textContent = 'Editing…';
    clearTimeout(saveTimer);
    var slug = current.slug;
    saveTimer = setTimeout(function () {
      api('PUT', '/api/lessons/' + slug + '/code', code.value)
        .then(function () { if (current && current.slug === slug) saveState.textContent = 'Saved'; })
        .catch(function (e) { saveState.textContent = 'Not saved: ' + e.message; });
    }, 600);
  }

  function showLesson(slug) {
    return api('GET', '/api/lessons/' + slug).then(function (l) {
      current = l; remember(slug); select.value = slug;
      var chips = ['<span id="lesson-status" class="chip status"></span>', '<span class="chip">' + esc(l.meta.minutes || '?') + ' min</span>']
        .concat((l.meta.concepts || []).map(function (c) { return '<span class="chip">' + esc(c) + '</span>'; }));
      $('lesson').innerHTML = '<div class="meta">' + chips.join('') + '</div>' + window.renderMarkdown(l.body);
      $('file-name').textContent = l.file;
      updateStats(l.stats);
      code.value = l.code; updateGutter(); code.scrollTop = 0;
      saveState.textContent = l.saved ? 'Your saved code' : 'Starter code';
      var st = l.stats, last = st && st.last_ts ? '<p class="muted small">Last run ' + esc(ago(st.last_ts)) + ': ' +
        (st.last_result === 'pass' ? 'all tests passed.' : 'not all tests passed yet.') + '</p>' : '';
      var review = st && st.due_now ? '<p><strong>Due for review.</strong> Press <strong>Reset</strong> to start again from the starter code, then solve it from memory.</p>' : '';
      results.innerHTML = review + last + '<p class="muted">Press <kbd>Ctrl</kbd>+<kbd>Enter</kbd> (or <kbd>⌘</kbd>+<kbd>Enter</kbd>) to run the tests.</p>' + tutorBox();
      wireTutor();
    });
  }

  function renderResult(r) {
    var html = [];
    var allOk = r.total > 0 && r.failed === 0 && !r.timedOut && r.compileErrors.length === 0;
    var pct = r.total ? Math.round(100 * r.passed / r.total) : 0;
    var headline = r.timedOut ? 'Timed out' :
      r.compileErrors.length ? 'Doesn’t compile yet' :
      r.total === 0 ? 'No tests ran' :
      allOk ? 'All ' + r.total + ' tests pass' : r.passed + ' of ' + r.total + ' tests pass';
    html.push('<div class="summary ' + (allOk ? 'ok' : 'bad') + '"><span>' + esc(headline) + '</span>' +
      '<span class="meter" aria-hidden="true"><span style="width:' + pct + '%"></span></span>' +
      '<span class="muted small">' + (r.durationMs / 1000).toFixed(1) + ' s</span></div>');
    r.compileErrors.forEach(function (d) {
      html.push('<div class="diag">' + (d.code ? '<span class="errcode">' + esc(d.code) + '</span> ' : '') +
        esc(d.title || '') + '<pre>' + esc(d.message) + '</pre></div>');
    });
    if (r.tests.length) {
      html.push('<ul class="tests">' + r.tests.map(function (t) {
        return '<li class="' + (t.ok ? 'ok' : 'bad') + '"><span>' + esc(t.name) + '</span>' +
          (t.ok || !t.message ? '' : '<pre>' + esc(t.message) + '</pre>') + '</li>';
      }).join('') + '</ul>');
    }
    if (r.stats) {
      var firstPass = allOk && r.stats.first_pass_run === r.stats.runs;
      html.push('<p class="muted small">Run ' + r.stats.runs + ' on this lesson' +
        (firstPass ? ', and your first full pass.' : r.stats.first_pass_run ? ' (first solved on run ' + r.stats.first_pass_run + ').' : '.') + '</p>');
    }
    if (allOk) {
      html.push('<div class="rate" id="rate"><span>How was that?</span>' +
        '<button type="button" class="ghost" data-rate="easy">Too easy</button>' +
        '<button type="button" class="ghost" data-rate="right">About right</button>' +
        '<button type="button" class="ghost" data-rate="hard">Too hard</button></div>');
    }
    html.push('<div id="next-card"></div>');
    if (!allOk) html.push(tutorBox());
    if (r.output) html.push('<details><summary>Full output</summary><pre>' + esc(r.output) + '</pre></details>');
    results.innerHTML = html.join('');
    showNext(r.next);
    wireTutor();
    var rate = $('rate');
    if (rate) rate.addEventListener('click', function (e) {
      var d = e.target.getAttribute('data-rate');
      if (!d || !current) return;
      api('POST', '/api/lessons/' + current.slug + '/feedback', d).then(function (res) {
        rate.innerHTML = '<span class="muted">Noted: ' + esc(e.target.textContent.toLowerCase()) + '. Your next step adapts to it.</span>';
        if (res.stats) updateStats(res.stats);
        showNext(res.next);
      });
    });
  }

  // The tutor (Claude): a hint for the current code, or a new drill aimed at what's hard.
  function tutorBox() {
    if (!tutor.enabled) return tutor.reason ? '<p class="muted small tutor-off">Tutor off: ' + esc(tutor.reason) + '</p>' : '';
    return '<div class="tutor"><input id="tutor-q" type="text" placeholder="Ask the tutor (optional)" aria-label="Question for the tutor">' +
      '<button type="button" id="hint-btn">Hint</button>' +
      '<button type="button" class="ghost" id="drill-btn" title="Claude writes a new drill aimed at what you find hard, and checks it before you see it">New drill on this</button></div>' +
      '<div id="tutor-out"></div>';
  }
  function wireTutor() {
    var hintBtn = $('hint-btn'), drillBtn = $('drill-btn'), out = $('tutor-out');
    if (!hintBtn) return;
    hintBtn.addEventListener('click', function () {
      if (!current) return;
      hintBtn.disabled = true; out.innerHTML = '<p class="muted">Thinking about your code…</p>';
      api('POST', '/api/lessons/' + current.slug + '/hint', JSON.stringify({ code: code.value, question: $('tutor-q').value }))
        .then(function (d) { out.innerHTML = '<div class="tutor-reply">' + window.renderMarkdown(d.hint) + '</div>'; out.scrollIntoView({ block: 'nearest' }); })
        .catch(function (e) { out.innerHTML = '<p class="summary bad">' + esc(e.message) + '</p>'; })
        .then(function () { hintBtn.disabled = false; });
    });
    drillBtn.addEventListener('click', function () {
      if (!current) return;
      drillBtn.disabled = true; hintBtn.disabled = true;
      out.innerHTML = '<p class="muted">Writing a new drill for you and checking it against a working solution. This takes about a minute…</p>';
      api('POST', '/api/lessons/' + current.slug + '/drill', '{}').then(function (d) {
        return api('GET', '/api/lessons').then(function (lessons) {
          lessonList = lessons;
          select.innerHTML = lessons.map(function (l) { return '<option value="' + esc(l.slug) + '">' + esc(optionLabel(l)) + '</option>'; }).join('');
          select.value = current.slug;
          out.innerHTML = '<div class="next"><div><strong>New drill:</strong> ' + esc(d.title) + '<div class="muted small">' + esc(d.targets) + '</div></div>' +
            '<button type="button" id="open-drill">Open</button></div>';
          $('open-drill').addEventListener('click', function () { showLesson(d.slug); });
          out.scrollIntoView({ block: 'nearest' });
        });
      }).catch(function (e) { out.innerHTML = '<p class="summary bad">' + esc(e.message) + '</p>'; })
        .then(function () { drillBtn.disabled = false; hintBtn.disabled = false; });
    });
  }

  // The adaptive next step: what to do after this, with a button to open it.
  function showNext(next) {
    var card = $('next-card');
    if (!card || !next) return;
    if (next.kind === 'done' || !next.slug) {
      card.innerHTML = '<p class="next"><strong>Next:</strong> ' + esc(next.reason) + '.</p>';
      return;
    }
    if (current && next.slug === current.slug && next.kind !== 'review') { card.innerHTML = ''; return; }
    var label = { review: 'Review', drill: 'Drill', stretch: 'Stretch', lesson: 'Next lesson' }[next.kind] || 'Next';
    card.innerHTML = '<div class="next"><div><strong>' + esc(label) + ':</strong> ' + esc(next.title) +
      '<div class="muted small">' + esc(next.reason) + '</div></div>' +
      '<button type="button" id="next-btn">Open</button></div>';
    $('next-btn').addEventListener('click', function () { showLesson(next.slug); });
  }

  function run() {
    if (!current || runBtn.disabled) return;
    runBtn.disabled = true; runBtn.textContent = 'Running…';
    results.innerHTML = '<p class="muted">Running tests…</p>';
    clearTimeout(saveTimer);
    api('POST', '/api/lessons/' + current.slug + '/run', code.value)
      .then(function (r) { if (r.stats) updateStats(r.stats); renderResult(r); saveState.textContent = 'Saved'; })
      .catch(function (e) { results.innerHTML = '<p class="summary bad">Couldn’t run the tests: ' + esc(e.message) + '</p>'; })
      .then(function () { runBtn.disabled = false; runBtn.textContent = 'Run tests'; });
  }

  runBtn.addEventListener('click', run);
  resetBtn.addEventListener('click', function () {
    if (!current) return;
    if (resetBtn.dataset.armed !== '1') {
      resetBtn.dataset.armed = '1'; resetBtn.textContent = 'Click again to reset';
      setTimeout(function () { resetBtn.dataset.armed = ''; resetBtn.textContent = 'Reset'; }, 3000);
      return;
    }
    resetBtn.dataset.armed = ''; resetBtn.textContent = 'Reset';
    api('DELETE', '/api/lessons/' + current.slug + '/code').then(function (d) {
      code.value = d.code; updateGutter(); saveState.textContent = 'Starter code';
      if (d.stats) updateStats(d.stats);
    });
  });
  select.addEventListener('change', function () { showLesson(select.value); });

  // Profile: shown in the lesson pane; the same text the app writes into PROGRESS.md.
  profileBtn.addEventListener('click', function () {
    api('GET', '/api/profile').then(function (d) {
      var md = d.markdown.split('\n').filter(function (l) { return l.indexOf('<!--') !== 0; }).join('\n');
      $('lesson').innerHTML = '<p><button id="back-btn" class="ghost" type="button">← Back to the lesson</button></p>' +
        window.renderMarkdown(md) + '<p class="muted small">This is also written to <code>PROGRESS.md</code> after every run. Commit it so Claude sessions can see your progress.</p>';
      $('back-btn').addEventListener('click', function () { if (current) showLesson(current.slug); });
    }).catch(function (e) { $('lesson').innerHTML = '<p class="summary bad">Couldn’t load your profile: ' + esc(e.message) + '</p>'; });
  });

  api('GET', '/api/info').then(function (info) {
    document.title = info.title;
    tutor = info.tutor || tutor;
    $('track-title').textContent = info.title;
    if (info.accent) document.documentElement.style.setProperty('--accent', info.accent);
    return api('GET', '/api/lessons');
  }).then(function (lessons) {
    lessonList = lessons;
    select.innerHTML = lessons.map(function (l) {
      return '<option value="' + esc(l.slug) + '">' + esc(optionLabel(l)) + '</option>';
    }).join('');
    if (!lessons.length) { $('lesson').innerHTML = '<p>No lessons found in <code>lessons/</code>.</p>'; return; }
    // Open the adaptive next step (a due review, a drill, …); otherwise where you left off.
    return api('GET', '/api/profile').then(function (p) { return p.next; }, function () { return null; }).then(function (next) {
      var saved = recall();
      var start = next && next.slug ? next.slug : lessons.some(function (l) { return l.slug === saved; }) ? saved : lessons[0].slug;
      return showLesson(start);
    });
  }).catch(function (e) {
    $('lesson').innerHTML = '<p class="summary bad">Couldn’t reach the server: ' + esc(e.message) + '</p>';
  });
})();
