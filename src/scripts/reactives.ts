// import { reactive } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { fs, invoke } from '@tauri-apps/api'
import { resolve, resourceDir } from '@tauri-apps/api/path';
// import { ElNotification,ElMessage } from "element-plus/es/components";

// sidebar公共方法/值
const sidebarReactives = reactive({
  isCollapse: true,
  activeMenuId: "1-1",
  delay: 200,
  extendPadding: "",
  //获取鼠标点击消除遮罩
  changeThisCollapse() {
    let t: NodeJS.Timeout | null = null;
    let firstClick = !t;
    if (firstClick) {
      this.isCollapse = this.isCollapse ? false : true;
      this.extendPadding = this.isCollapse ? "" : "70";
    }
    if (t) {
      clearTimeout(t);
    }
    t = setTimeout(() => {
      t = null;
      if (!firstClick) {
        this.isCollapse = this.isCollapse ? false : true;
        this.extendPadding = this.isCollapse ? "" : "70";
      }
    }, this.delay);
  },
});

// topbar 公共方法/值
interface ImageProps {
  image_paths: [string];
  count: number;
}



// const tools = reactive({
//   message(msg: string) {
//     ElNotification({
//       message: msg,
//       type: "success",
//       title: "🐮----🍺",
//       position: "bottom-left",
//     });
//   },
// });

const resource_dir = ref("");
// resource_dir: "",
type UserDataType = {
  qulity: number,
  latestSelectedDirPath: string,
  latestSelectedOutputPath: string,
  autoUseBrand: boolean,
  brand: string,
  font: string,
  brands: Array<{value: string, label: string}>,
  renameSuffix: string,
  renamePreffix: string,
  renameCenter: string,
}

const user_conf = reactive({
  qulity: 85,
  latestSelectedDirPath: "",
  latestSelectedOutputPath: "",
  autoUseBrand: true,
  brand: "nikon",
  font: "",
  renameSuffix: "",
  renamePreffix: "",
  renameCenter: "",
  user_conf_path: "",
  brands: [{ value: 'canon', label: "佳能" }, { value: 'nikon', label: "尼康" }, { value: 'sony', label: "索尼" }, 
                {value: "panasonic", label: "松下"}, {value: "fujifilm", label: "富士"} ],

  async init_user_conf() {
    resource_dir.value = await resourceDir();
    this.user_conf_path = await resolve(resource_dir.value, "resources", "user.conf");
    const contents = await fs.readTextFile(this.user_conf_path);
    console.log("ccc:" + contents);
    if (contents == "") {
      console.log("kong");
    } else {
      console.log("kong1");
      let user_data: UserDataType = JSON.parse(contents);
      // let entries = Object.entries(user_data);
      // for (let i=0; i < entries.length; i++) {
      //   let key = entries[i][0]
      //   let k2 = "autoUseBrand"
      //   user_conf[k2] = entries[i][1];
      // }
      this.B2A(user_conf, user_data);

    }

  },
  // A.* <-- B.*
  B2A(A: any, B: any) {
    A.autoUseBrand = B.autoUseBrand;
    A.brand = B.brand;
    A.font = B.font;
    A.latestSelectedDirPath = B.latestSelectedDirPath;
    A.latestSelectedOutputPath = B.latestSelectedOutputPath;
    A.qulity = B.qulity;
    A.brands = B.brands;
    A.renameSuffix = B.renameSuffix;
    A.renamePreffix = B.renamePreffix;
    A.renameCenter = B.renameCenter;
  },

  async save_user_conf(baseForm: UserDataType) {
    let user_save: UserDataType = new Object as UserDataType;
    this.B2A(user_save, baseForm);
    this.B2A(user_conf, baseForm);
    let json_contents = JSON.stringify(user_save, null, 4);
    await fs.writeTextFile(this.user_conf_path, json_contents);
  },

  load_conf(baseForm: UserDataType) {
    this.B2A(baseForm, user_conf);
  }


})



const image_progress = reactive({
  status: false,
  value: 90,
  count: { completed: 0, total: 0 },
  image_paths: { count: 0, image_paths: [""] },
  image_dir_path: "",
  increase() {
    if (this.value <= 98) {
      this.value += 2;
      if (this.value < 0) {
        this.value = 0;
      }
    }
  },
  //

  status_toogle() {
    this.status = !this.status;
    // console.log(this.status);
  },

  reset_progress() {
    this.value = 0;
    this.count.completed = 0;
    this.count.total = 0;
    this.image_paths = { count: 0, image_paths: [""] };
  },
  increase_one() {
    this.count.completed++;
    if (this.count.total > 0) {
      let value = this.count.completed / this.count.total;
      if (value < 0) {
        this.value = 0;
      } else if (value > 100) {
        this.value = 100;
      } else {
        this.value = Math.round((value + Number.EPSILON) * 10000) / 100;
      }
    }
  },
  update_progress(completed: number, total: number) {
    this.count.completed = completed;
    this.count.total = total;
    if (total > 0) {
      let value = completed / total;
      if (value < 0) {
        value = 0;
      } else if (value > 100) {
        value = 100;
      }
      this.value = Math.round((value + Number.EPSILON) * 10000) / 100;
    }
    // color();
  },

  //
  dragFiles(arr: Array<string>) {
    this.image_paths = {
      count: arr.length,
      image_paths: arr,
    } as ImageProps;
    image_progress.update_progress(0, arr.length);
    elmessage("selected: " + this.image_paths);
  },

  async selectFiles() {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Image",
          extensions: ["jpg", "jpeg"],
        },
      ],
    });
    if (Array.isArray(selected) && selected?.length != 0) {
      // console.log(selected);
      this.image_paths = {
        count: selected.length,
        image_paths: selected,
      } as ImageProps;
      image_progress.update_progress(0, selected.length);
      elmessage("selected: " + this.image_paths);
      // user selected multiple files
    } else if (selected === null) {
      // user cancelled the selection
      ElMessage({
        message: "null file selected.",
        type: "warning",
      });
    } else if (typeof selected === "string") {
      // console.log("single fil: " + selected);
      this.image_paths = { count: 1, image_paths: [selected] };
      // this.message("handle_json: " + handle_json.count);
      //   await process_single_image(handle_json);
      image_progress.update_progress(0, 1);
      elmessage("selected: " + this.image_paths);
    }
  },

  async update_user_data2BD(key: string, value: string) {
    let res = await invoke("handle_front_update_key", {
      key: key,
      value: value,
    });
    elmessage("update output dir: " + res);
  },
  //

  async selectDirs() {
    const selected = await open({
      directory: true,
      multiple: false,
      // defaultPath: await appDir(),
    });
    if (selected === null) {
      // user cancelled the selection
      ElMessage({
        message: "null dir selected.",
        type: "warning",
      });
    } else if (typeof selected == "string") {
      console.log("selected single dir " + selected);
      elmessage("selected single dir " + selected);
      // this.update_user_data2BD("output_dir", selected);
      this.image_dir_path = selected;
    }
  },


  async selectOutputDirs() {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: "/Users/fly/Downloads",
    });
    if (selected === null) {
      // user cancelled the selection
      ElMessage({
        message: "null dir selected.",
        type: "warning",
      });
    } else if (typeof selected == "string") {
      console.log("selected single dir " + selected);
      elmessage("selected single dir " + selected);
      this.update_user_data2BD("output_dir", selected);
    }
  },

  async process_image() {
    if ((this.count.total != 0) && (this.count.completed != this.count.total)) {
      let send_content = JSON.stringify(this.image_paths);
      console.log(send_content);
      let res = await invoke("handle_front_select_files", { imagesObj: this.image_paths });
      elmessage("处理文件: " + res);
    } else if (this.image_dir_path != "") {
      console.log(this.image_dir_path);
      let res = await invoke("handle_front_select_dir", { imageDir: this.image_dir_path });
      elmessage("处理目录: " + res);

    } else {
      elmessage("未选择文件或已完成");

    }

  },

  //
});

function elmessage(msg: string) {
  ElNotification({
    message: msg,
    type: "success",
    title: "🐮----🍺",
    position: "bottom-left",
  });
}

//previewwidget 公共
const previewwidget = reactive({
  inputValue: false
})
user_conf.init_user_conf();


onMounted(() => {
  console.log("mounted init");
  user_conf.init_user_conf();
})

export { image_progress, sidebarReactives, previewwidget, elmessage, user_conf }; 
export type { UserDataType };

