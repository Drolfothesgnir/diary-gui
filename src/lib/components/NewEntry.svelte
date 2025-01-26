<script lang="ts">
  export let onNewEntryClick: () => void;
  export let onDumpEntriesClick: () => void;
  export let dumpEntriesDisabled: boolean;
</script>

<div class="newEntry">
  <button class="new-entry" on:click={onNewEntryClick}>New Entry</button>
  <button
    class="dump-entries"
    on:click={onDumpEntriesClick}
    disabled={dumpEntriesDisabled}>Dump Entries</button
  >
</div>

<style lang="scss">
  .newEntry {
    width: 100%;
    display: flex;
    justify-content: center;
    padding: 1rem;
    border-top: 3px solid #b0c4de;

    /* Стиль кнопки "Новая запись" */
    button {
      border: none; /* Без границы */
      padding: 10px 20px; /* Внутренние отступы для удобного размера кнопки */
      border-radius: 10px; /* Округлённые углы */
      box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); /* Лёгкая тень */
      font-size: 16px; /* Размер текста */
      transition: 0.3s ease; /* Плавный переход фона при наведении */

      &.new-entry {
        color: #4f4f4f; /* Тёмно-серый текст */
        background-color: #98fb98; /* Светло-зелёный фон */

        &:hover {
          background-color: #90ee90; /* Более тёмный оттенок зелёного */
          cursor: pointer; /* Изменение курсора на указатель */
        }
      }

      &.dump-entries {
        margin-left: 20px;
        background-color: #ffcc80;

        &:hover {
          background-color: #ffa726;
          color: #fff;
        }

        &:disabled {
          background-color: #f8f7f8;
          color: #b0c4de;
          cursor: not-allowed;
        }
      }
    }
  }
</style>
