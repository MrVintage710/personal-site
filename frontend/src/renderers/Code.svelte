<script>
    import hljs from 'highlight.js';

    export let lang;
    export let text;
    
    const html = hljs.highlight(text, {language : lang}).value;
  </script>
  
  <pre><code class={lang + " hljs "}>{@html html}</code></pre>