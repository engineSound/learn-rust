// Minimal Markdown renderer for lesson.md files (shared by both apps).
// Supports headings, paragraphs, fenced code, tables, lists, inline code,
// bold, italic and links. Lessons are written by us, but text is still escaped.
(function () {
  function esc(s) {
    return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  }
  function inline(s) {
    var parts = s.split(/(`[^`]*`)/);
    return parts.map(function (p) {
      if (p.length > 1 && p[0] === '`' && p[p.length - 1] === '`') return '<code>' + esc(p.slice(1, -1)) + '</code>';
      return esc(p)
        .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
        .replace(/(^|[^*])\*([^*]+)\*/g, '$1<em>$2</em>')
        .replace(/\[([^\]]+)\]\((https?:[^)\s]+)\)/g, '<a href="$2" target="_blank" rel="noopener">$1</a>');
    }).join('');
  }
  function cells(line) {
    return line.trim().replace(/^\|/, '').replace(/\|$/, '').split('|').map(function (c) { return c.trim(); });
  }
  function render(md) {
    var lines = md.replace(/\r\n/g, '\n').split('\n');
    var out = [], i = 0;
    while (i < lines.length) {
      var line = lines[i];
      if (/^```/.test(line)) {
        var buf = []; i++;
        while (i < lines.length && !/^```/.test(lines[i])) buf.push(lines[i++]);
        i++;
        out.push('<pre><code>' + esc(buf.join('\n')) + '</code></pre>');
      } else if (/^#{1,6} /.test(line)) {
        var level = line.match(/^#+/)[0].length;
        out.push('<h' + level + '>' + inline(line.slice(level + 1)) + '</h' + level + '>');
        i++;
      } else if (/^\s*\|/.test(line) && i + 1 < lines.length && /^\s*\|?\s*:?-+/.test(lines[i + 1])) {
        var head = cells(line); i += 2;
        var rows = [];
        while (i < lines.length && /^\s*\|/.test(lines[i])) rows.push(cells(lines[i++]));
        out.push('<table><thead><tr>' + head.map(function (c) { return '<th>' + inline(c) + '</th>'; }).join('') +
          '</tr></thead><tbody>' + rows.map(function (r) {
            return '<tr>' + r.map(function (c) { return '<td>' + inline(c) + '</td>'; }).join('') + '</tr>';
          }).join('') + '</tbody></table>');
      } else if (/^\s*([-*]|\d+\.) /.test(line)) {
        var ordered = /^\s*\d+\./.test(line), items = [];
        while (i < lines.length && /^\s*([-*]|\d+\.) /.test(lines[i])) items.push(lines[i++].replace(/^\s*([-*]|\d+\.) /, ''));
        var tag = ordered ? 'ol' : 'ul';
        out.push('<' + tag + '>' + items.map(function (t) { return '<li>' + inline(t) + '</li>'; }).join('') + '</' + tag + '>');
      } else if (line.trim() === '') {
        i++;
      } else {
        var para = [];
        while (i < lines.length && lines[i].trim() !== '' && !/^(```|#{1,6} |\s*\||\s*([-*]|\d+\.) )/.test(lines[i])) para.push(lines[i++]);
        out.push('<p>' + inline(para.join(' ')) + '</p>');
      }
    }
    return out.join('\n');
  }
  window.renderMarkdown = render;
})();
