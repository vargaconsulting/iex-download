// Load this BEFORE the MathJax CDN script in mkdocs.yml
window.MathJax = {
  tex: {
    // include the delimiters you actually use:
    inlineMath: [['\\(', '\\)'], ['$', '$']],
    displayMath: [['\\[', '\\]'], ['$$', '$$']],
    processEscapes: true,
    processEnvironments: true
  },
  options: {
    // With arithmatex(generic:true), content is wrapped in .arithmatex
    processHtmlClass: 'arithmatex',
    ignoreHtmlClass: '.*'   // process only .arithmatex
  },
  startup: {
    // we'll control typesetting manually to avoid races
    typeset: false
  }
};

// Re-typeset after every MkDocs Material page swap
if (window.document$) {
  document$.subscribe(() => {
    const container = document.querySelector('.md-content'); // main article
    // Defer to next animation frame so DOM is fully replaced
    requestAnimationFrame(() => {
      MathJax.typesetClear();                      // clear previous MathItems
      MathJax.typesetPromise([container]);         // typeset current page
    });
  });
}
