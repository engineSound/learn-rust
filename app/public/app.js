// Front end shared by learn-nodejs and learn-rust. Both servers expose the same API:
//   GET  /api/info                      -> { track, title, accent }
//   GET  /api/lessons                   -> [{ slug, id, title, minutes, concepts }]
//   GET  /api/lessons/:slug             -> { slug, meta, body, file, code }
//   PUT  /api/lessons/:slug/code        (text body) -> { saved: true }
//   DELETE /api/lessons/:slug/code      -> { code }  (back to the starter)
//   POST /api/lessons/:slug/run         (text body) -> run result
(function () {
  var $ = function (id) { return document.getElementById(id); };
  var select = $('lesson-select'), code = $('code'), gutter = $('gutter');
  var runBtn = $('run-btn'), resetBtn = $('reset-btn'), results = $('results'), saveState = $('save-state');
  var current = null, saveTimer = null;

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
      var chips = ['<span class="chip">' + esc(l.meta.minutes || '?') + ' min</span>']
        .concat((l.meta.concepts || []).map(function (c) { return '<span class="chip">' + esc(c) + '</span>'; }));
      $('lesson').innerHTML = '<div class="meta">' + chips.join('') + '</div>' + window.renderMarkdown(l.body);
      $('file-name').textContent = l.file;
      code.value = l.code; updateGutter(); code.scrollTop = 0;
      saveState.textContent = l.saved ? 'Your saved code' : 'Starter code';
      results.innerHTML = '<p class="muted">Press <kbd>Ctrl</kbd>+<kbd>Enter</kbd> (or <kbd>⌘</kbd>+<kbd>Enter</kbd>) to run the tests.</p>';
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
    if (allOk) html.push('<p>Lesson complete. Add a line to <code>PROGRESS.md</code>: what tripped you up?</p>');
    if (r.output) html.push('<details><summary>Full output</summary><pre>' + esc(r.output) + '</pre></details>');
    results.innerHTML = html.join('');
  }

  function run() {
    if (!current || runBtn.disabled) return;
    runBtn.disabled = true; runBtn.textContent = 'Running…';
    results.innerHTML = '<p class="muted">Running tests…</p>';
    clearTimeout(saveTimer);
    api('POST', '/api/lessons/' + current.slug + '/run', code.value)
      .then(function (r) { renderResult(r); saveState.textContent = 'Saved'; })
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
    });
  });
  select.addEventListener('change', function () { showLesson(select.value); });

  api('GET', '/api/info').then(function (info) {
    document.title = info.title;
    $('track-title').textContent = info.title;
    if (info.accent) document.documentElement.style.setProperty('--accent', info.accent);
    return api('GET', '/api/lessons');
  }).then(function (lessons) {
    select.innerHTML = lessons.map(function (l) {
      return '<option value="' + esc(l.slug) + '">' + esc(l.title) + ' · ' + esc(l.minutes || '?') + ' min</option>';
    }).join('');
    if (!lessons.length) { $('lesson').innerHTML = '<p>No lessons found in <code>lessons/</code>.</p>'; return; }
    var saved = recall();
    var start = lessons.some(function (l) { return l.slug === saved; }) ? saved : lessons[0].slug;
    return showLesson(start);
  }).catch(function (e) {
    $('lesson').innerHTML = '<p class="summary bad">Couldn’t reach the server: ' + esc(e.message) + '</p>';
  });
})();
