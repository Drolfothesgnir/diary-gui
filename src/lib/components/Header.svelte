<script lang="ts">
  import type { ChangeEventHandler, EventHandler } from "svelte/elements";
  import { PinnedFilter, SortOrder } from "../../types";

  type Props = {
    search: string;
    pinned: PinnedFilter;
    sort: SortOrder;
    onClearClicked: () => void;
    onSearchChanged: ChangeEventHandler<HTMLInputElement>;
    onPinnedChange: (pinnedFilter: PinnedFilter) => void;
    onSortOrderChange: (sortOrder: SortOrder) => void;
    onSubmit: EventHandler<SubmitEvent, HTMLFormElement>;
  };
  let {
    search,
    pinned,
    sort,
    onClearClicked,
    onSearchChanged,
    onPinnedChange,
    onSortOrderChange,
    onSubmit,
  }: Props = $props();

  const pinnedOptions = [
    {
      value: PinnedFilter.ALL,
      label: "All entries",
    },
    {
      value: PinnedFilter.PINNED,
      label: "Pinned entries",
    },
    {
      value: PinnedFilter.UNPINNED,
      label: "Unpinned entries",
    },
  ];

  const pinnedChangeHandler: ChangeEventHandler<HTMLSelectElement> = (e) => {
    switch (e.currentTarget.value) {
      case "ALL":
        return PinnedFilter.ALL;

      case "PINNED":
        return PinnedFilter.PINNED;

      case "UNPINNED":
        return PinnedFilter.UNPINNED;

      default:
        return PinnedFilter.ALL;
    }
  };

  const sortOptions = [
    {
      value: SortOrder.ASC,
      label: "Oldest",
    },
    {
      value: SortOrder.DESC,
      label: "Newest",
    },
  ];

  const sortOrderChangeHandler: ChangeEventHandler<HTMLSelectElement> = (e) => {
    switch (e.currentTarget.value) {
      case "ASC":
        return SortOrder.ASC;

      case "DESC":
        return SortOrder.DESC;

      default:
        return SortOrder.DESC;
    }
  };
</script>

<header class="header">
  <div class="header-filters">
    <form onsubmit={onSubmit} class="search-controls">
      <button class="clear-button" onclick={onClearClicked}>Clear</button>
      <div class="string-search">
        <input
          type="text"
          class="search-input"
          bind:value={search}
          onchange={onSearchChanged}
          placeholder="Search for a substring"
        />
      </div>
      <div class="pinned-filter">
        <select
          class="search-select"
          bind:value={pinned}
          onchange={(e) => onPinnedChange(pinnedChangeHandler(e))}
        >
          {#each pinnedOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
      <div class="sort-order">
        <select
          class="search-select"
          bind:value={sort}
          onchange={(e) => onSortOrderChange(sortOrderChangeHandler(e))}
        >
          {#each sortOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
      <button class="search-button">Search</button>
    </form>
  </div>
</header>

<style lang="scss">
  .header {
    padding: 1rem;
    color: #fff;
    background-color: #b3d4fc;
    display: flex;
    justify-content: center;
  }

  .search-controls {
    padding: 1rem;
    background: #f8f9fa;
    border: 1px solid #e9ecef;
    border-radius: 8px;
    display: flex;
    gap: 12px;
    align-items: center;
    flex-wrap: wrap;
  }

  .clear-button {
    background-color: #dc3545;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 14px;
    line-height: 1.5;
    cursor: pointer;
    transition: background-color 0.15s ease-in-out;

    &:hover {
      background-color: #bb2d3b;
    }

    &:active {
      background-color: #a52834;
    }

    &:focus {
      outline: none;
      box-shadow: 0 0 0 3px rgba(220, 53, 69, 0.25);
    }
  }

  .search-input {
    background-color: white;
    border: 1px solid #ced4da;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 14px;
    line-height: 1.5;
    flex: 1;
    min-width: 200px;

    &:hover {
      border-color: #adb5bd;
    }

    &:focus {
      outline: none;
      border-color: #86b7fe;
      box-shadow: 0 0 0 3px rgba(13, 110, 253, 0.25);
    }
  }

  .search-select {
    appearance: none;
    background-color: white;
    border: 1px solid #ced4da;
    border-radius: 4px;
    padding: 6px 24px 6px 12px;
    font-size: 14px;
    line-height: 1.5;
    background-image: url("data:image/svg+xml;charset=utf-8,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23495057' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
    background-size: 16px;
    min-width: 120px;

    &:hover {
      border-color: #adb5bd;
    }

    &:focus {
      outline: none;
      border-color: #86b7fe;
      box-shadow: 0 0 0 3px rgba(13, 110, 253, 0.25);
    }
  }

  .search-button {
    background-color: #0d6efd;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 14px;
    line-height: 1.5;
    cursor: pointer;
    transition: background-color 0.15s ease-in-out;

    &:hover {
      background-color: #0b5ed7;
    }

    &:active {
      background-color: #0a58ca;
    }

    &:focus {
      outline: none;
      box-shadow: 0 0 0 3px rgba(13, 110, 253, 0.25);
    }
  }
</style>
