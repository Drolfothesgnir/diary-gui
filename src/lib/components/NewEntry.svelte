<script lang="ts">
  export let onClick: () => void;
</script>

<div class="newEntry">
  <button on:click={onClick}>New Entry</button>
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
      background-color: #98fb98; /* Светло-зелёный фон */
      color: #4f4f4f; /* Тёмно-серый текст */
      border: none; /* Без границы */
      padding: 10px 20px; /* Внутренние отступы для удобного размера кнопки */
      border-radius: 10px; /* Округлённые углы */
      box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); /* Лёгкая тень */
      font-size: 16px; /* Размер текста */
      transition: background-color 0.3s ease; /* Плавный переход фона при наведении */

      &:hover {
        background-color: #90ee90; /* Более тёмный оттенок зелёного */
        cursor: pointer; /* Изменение курсора на указатель */
      }
    }
  }
</style>
