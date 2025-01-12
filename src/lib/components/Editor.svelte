<script lang="ts">
  import { onMount, afterUpdate } from "svelte";

  export let initialContent = "";
  export let onContentSave;
  export let onCloseClick;
  export let focusOnMount = true;

  let textareaRef: HTMLTextAreaElement;

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter" && e.shiftKey) {
      onContentSave(textareaRef.value || "");
    }
  };

  const onSaveClick = () => {
    onContentSave(textareaRef.value || "");
  };

  // This will run when the component mounts
  onMount(() => {
    textareaRef.value = initialContent;

    if (focusOnMount) {
      textareaRef.focus();
    }
  });

  // Whenever initialContent changes, we update the textarea
  afterUpdate(() => {
    textareaRef.value = initialContent;
  });
</script>

<!-- HTML Template -->
<div class="editor">
  <div class="inputBlock">
    <div class="inputWrapper">
      <button class="closeButton" on:click={onCloseClick}> &times; </button>
      <textarea
        spellcheck
        bind:this={textareaRef}
        on:keydown={handleKeyDown}
        placeholder="Write your diary entry here... "
      ></textarea>
    </div>
    <div class="inputBlockBottom">
      <span>Shift + Enter to submit</span>
      <button on:click={onSaveClick}>Save</button>
    </div>
  </div>
</div>

<style lang="scss">
  .editor {
    display: flex;
    justify-content: center;
    padding-bottom: 1rem;
    padding-top: 1rem;
    border-top: 3px solid #b0c4de;
    background-color: #fafafa;

    .inputBlock {
      width: 50%;

      .inputWrapper {
        position: relative;

        .closeButton {
          position: absolute;
          right: 8px;
          top: 8px;
          font-size: 25px;
          line-height: 20px;
          color: #707070;
          width: 20px;
          height: 20px;
          transition: 0.3s;

          &:hover {
            color: #8b0000;
          }
        }

        textarea {
          width: 100%;
          height: 300px;
          padding: 1rem;
          resize: none;
          background-color: #f0f8ff;
          border: 5px solid #b0c4de;
          color: #4f4f4f;
          font-family: "Georgia Regular";
          font-size: 16px;
          display: block;
          margin-bottom: 10px;
          box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1); /* Лёгкая тень */
          border-radius: 8px; /* Округление углов */
          transition: box-shadow 0.3s ease; /* Плавный переход тени при фокусе */

          &:focus {
            outline: none; /* Убираем стандартный чёрный outline */
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2); /* Усиленная тень при фокусе */
            border: 5px solid #87ceeb; /* Голубая граница при фокусе */
          }
        }
      }

      .inputBlockBottom {
        display: flex;
        justify-content: space-between;

        span {
          display: block;
          padding: 0.2rem;
          font-size: 20px;
        }

        button {
          display: block;
          padding: 0.2rem;
          font-size: 20px;
          background-color: #98fb98;
          color: #006400;
        }
      }
    }
  }
</style>
