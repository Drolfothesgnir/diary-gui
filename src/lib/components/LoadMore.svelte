<script lang="ts">
  export let onClick: () => void;
</script>

<div class="loadMore">
  <button on:click={onClick}>Load more</button>
</div>

<style lang="scss">
  .loadMore {
    width: 100%;
    display: flex;
    justify-content: center;
    padding: 1rem;

    /* Стиль кнопки "Новая запись" */
    button {
      background-color: #b3d4fc; /* Светло-зелёный фон */
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
