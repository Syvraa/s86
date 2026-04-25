<script lang="ts">
  import ace from "ace-builds";
  import "ace-builds/src-noconflict/theme-monokai";
  import { onMount } from "svelte";

  let editor: ace.Editor;
  let editorDiv: HTMLDivElement;
  let prevMarkerId: number | null = null;

  onMount(() => {
    editor = ace.edit(editorDiv);
    editor.setTheme("ace/theme/monokai");
    editor.setFontSize("1.1em");
  });

  export function getContent() {
    return editor.getValue();
  }

  export function highlightLine(line: number | undefined) {
    removeLastMarker();

    if (line) {
      prevMarkerId = editor.session.addMarker(
        new ace.Range(line - 1, 0, line - 1),
        "editorhighlight",
        "fullLine",
        true,
      );
    } else {
      removeLastMarker();
    }
  }

  function removeLastMarker() {
    if (prevMarkerId) {
      editor.session.removeMarker(prevMarkerId);
    }
  }

  export function reset() {
    removeLastMarker();
  }
</script>

<div class="editor" bind:this={editorDiv}></div>

<style>
  /* If it's not global then it gets removed and the highlighting won't work. */
  :global(.editorhighlight) {
    position: absolute;
    background-color: rgba(0, 0, 255, 0.3);
    z-index: 100;
  }

  .editor {
    position: absolute;
    margin: 0px;
    width: 100%;
    height: 100%;
  }
</style>
