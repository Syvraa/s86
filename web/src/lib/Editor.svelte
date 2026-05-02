<script lang="ts">
  import ace from "ace-builds";
  import "ace-builds/src-noconflict/theme-monokai";
  import type { SimulatorError, SyntaxError } from "s86-lib";
  import { onMount } from "svelte";
  import { SIMULATOR_ERRORS, SYNTAX_ERRORS } from "../errors";

  let { onchange }: { onchange: () => void } = $props();

  let editor: ace.Editor;
  let editorDiv: HTMLDivElement;
  let prevMarkerId: number | null = null;

  onMount(() => {
    editor = ace.edit(editorDiv);
    editor.setTheme("ace/theme/monokai");
    editor.setFontSize("1.1em");
    editor.on("change", onchange);
  });

  export function getContent() {
    return editor.getValue();
  }

  export function highlightLine(line: number | undefined): void;
  function highlightLine(line: number, error: boolean): void;
  export function highlightLine(line: number | undefined, error?: boolean) {
    removeLastMarker();

    if (line) {
      prevMarkerId = editor.session.addMarker(
        new ace.Range(line - 1, 0, line - 1),
        error ? "editorerrorhighlight" : "editorhighlight",
        "fullLine",
        true,
      );
    } else {
      removeLastMarker();
    }
  }

  export function highlightSimulatorError(line: number, error: SimulatorError) {
    highlightLine(line, true);
    editor
      .getSession()
      .setAnnotations([{ row: line - 1, column: 0, text: SIMULATOR_ERRORS[error], type: "error" }]);
  }

  function removeLastMarker() {
    if (prevMarkerId) {
      editor.session.removeMarker(prevMarkerId);
    }
  }

  export function reset() {
    removeLastMarker();
    editor.getSession().clearAnnotations();
  }

  export function showErrors(errors: SyntaxError[]) {
    editor.getSession().setAnnotations(
      errors.map((err) => {
        return {
          row: err.line - 1,
          column: 0,
          text: SYNTAX_ERRORS[err.error],
          type: "error",
        };
      }),
    );
  }

  export function clearErrors() {
    showErrors([]);
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

  :global(.editorerrorhighlight) {
    position: absolute;
    background-color: rgba(255, 0, 0, 0.3);
    z-index: 101;
  }

  .editor {
    position: absolute;
    margin: 0px;
    width: 100%;
    height: 100%;
  }
</style>
