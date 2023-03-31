<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import File from "./lib/File.svelte";

  const fonts = [
    "咬",
    "哟",
    "哪",
    "咳",
    "咤",
    "咪",
    "咩",
    "罚",
    "炭",
    "峙",
    "峡",
    "品",
    "蚂",
    "咽",
    "蛊",
    "勋",
    "哗",
    "骂",
    "响",
    "哆",
    "咱",
    "哈",
    "趴",
    "昭",
    "蚁",
    "虹",
    "思",
    "畏",
    "虽",
    "虾",
    "胃",
    "界",
    "贵",
    "昵",
    "哑",
    "咧",
    "显",
    "哄",
    "眨",
    "昨",
    "映",
    "星",
    "冒",
    "哇",
    "盼",
    "盹",
    "昧",
    "竖",
    "尝",
    "削",
    "是",
    "览",
    "临",
    "省",
    "虐",
    "韭",
    "毖",
    "轻",
    "皆",
    "鸦",
    "轶",
    "战",
    "背",
    "轴",
    "轳",
    "点",
    "牵",
    "鸥",
    "耍",
    "残",
    "耷",
    "奎",
    "殃",
    "殆",
    "耐",
    "轱",
    "面",
  ];
  let res_index = -1;
  let pic_url = "/pic/font.jpg";
  let p = "font.jpg";
  let text = "";
  let indexs = [];
  let input;
  let image;
  let showImage = false;

  function reduce_one() {
    if (res_index >= 0) {
      res_index--;
    }
  }
  function add_one() {
    if (res_index < 5) {
      res_index++;
    }
  }
  async function cvdemo() {
    text = "";
    indexs = await invoke("cv_demo", { p });
    indexs.forEach((element) => {
      if (fonts[element] != undefined) {
        text = fonts[element] + " " + text;
      }
    });
  }

  function onChange() {
    const file = input.files[0];

    if (file) {
      showImage = true;

      const reader = new FileReader();
      reader.addEventListener("load", function () {
        image.setAttribute("src", reader.result);
      });
      reader.readAsDataURL(file);
      p = file.name;
      res_index = -1;
      file.return;
    }
    showImage = false;
  }

  let t = "<";
  let r = ">";
</script>

<main class="container">
  <div class="row ">
    <div class="column">
      <h3>图片</h3>
      <input bind:this={input} on:change={onChange} type="file" />
      <div class="column">
        {#if res_index > -1 && res_index <= 5}
          <img src="/res/{res_index}.jpg" class="logo vite" alt="lapis" />
          <!-- <img bind:this={image} src="" class="logo vite" alt="lapis" /> -->
        {:else}
          <img src="/pic/{p}" class="logo vite" alt="Vite Logo" />
          <!-- <img bind:this={image} src="" class="logo vite" alt="Vite Logo" /> -->
        {/if}
      </div>
      <div class="row">
        <button on:click={reduce_one}> {t} </button>
        <button on:click={add_one}> {r} </button>
      </div>
    </div>
    <div class="but">
      <button on:click={cvdemo}>转换</button>
    </div>
    <div class="column ">
      <h3>文字</h3>
      <textarea class="textarea-css" bind:value={text} />
    </div>
  </div>
</main>

<style>
  .textarea-css {
    height: 300px;
    font-size: large;
    font-weight: 600;
  }
  .but {
    display: flex;
    align-items: center;
    margin: 0px 40px;
  }
  .logo.vite {
    width: 30em;
    height: auto;
  }
  .logo.vite:hover {
    filter: drop-shadow(0 0 1em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
