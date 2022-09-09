// import { reactive } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { fs, invoke } from '@tauri-apps/api'
import { pictureDir, resolve, resourceDir } from '@tauri-apps/api/path';
// import { ElNotification,ElMessage } from "element-plus/es/components";
import { emit, listen } from "@tauri-apps/api/event";
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

interface UserSettings {
  output_dir: string,
  qulity: number,
  auto_user_brand: boolean,
  brand: string,
  filename_pattern: Array<string>,
}

type RenameType = {
  SD: Array<{ id: number, value: string, label: string }>,
  value: { id: number, value: string, label: string },
  input: string,
  valid: boolean
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
  brands: Array<{ value: string, label: string }>,
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
  { value: "panasonic", label: "松下" }, { value: "fujifilm", label: "富士" }],

  async init_user_conf() {
    // TODO: check resources file ...如果不存在提示警告。
    resource_dir.value = await resourceDir();
    this.user_conf_path = await resolve(resource_dir.value, "resources", "user.conf");
    const contents = await fs.readTextFile(this.user_conf_path);
    // console.log("ccc:" + contents);
    if (contents == "") {
      // 初始化数据：=== reset 
      // load 默认路径 为：const pictureDirPath = await pictureDir();
      // console.log("105")

    } else {
      // console.log("kong1");
      let user_data_load: UserDataType = JSON.parse(contents);
      // let entries = Object.entries(user_data);
      // for (let i=0; i < entries.length; i++) {
      //   let key = entries[i][0]
      //   let k2 = "autoUseBrand"
      //   user_conf[k2] = entries[i][1];
      // }
      if (user_data_load.latestSelectedOutputPath == ""){
        user_data_load.latestSelectedOutputPath = await pictureDir();
      }
      this.B2A(user_conf, user_data_load);
      // TODO 前后端初始化 并且发送数据给后端；
      // user_conf.update_user_data2BD("output_dir", pictureDirPath);
      let update_data_send: UserSettings = {
        output_dir: this.latestSelectedOutputPath,
        qulity: this.qulity,
        auto_user_brand: this.autoUseBrand,
        brand: this.brand,
        filename_pattern: [this.renamePreffix, this.renameCenter, this.renameSuffix]
      };
      console.log("init send data:" );
      console.log(update_data_send);
      // send to backend.
      let res: string = await invoke("handle_front_update_user_data", { userData: update_data_send });


    }

  },
  // A.* <-- B.*
  B2A(A: UserDataType, B: UserDataType) {
    const properties = ["autoUseBrand", "brand", "font", "latestSelectedDirPath",
      "latestSelectedOutputPath", "qulity", "brands", "renameSuffix", "renamePreffix", "renameCenter"]
    properties.forEach((ele) => {
      if (B[ele] != null || B[ele] == "") {
        A[ele] = B[ele];
      }
    });
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
  },

  async selectOutputDirs() {
    
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: this.latestSelectedOutputPath,
    });
    if (selected === null) {
      // user cancelled the selection
      ElMessage({
        message: "null dir selected.",
        type: "warning",
      });
    } else if (typeof selected == "string") {
      this.latestSelectedOutputPath = selected;
      // console.log("selected single dir " + selected);
      elmessage("selected single dir " + selected);
      this.update_user_data2BD("output_dir", selected);
    }
  },

  async update_user_data2BD(key: string, value: string) {
    let res = await invoke("handle_front_update_key", {
      key: key,
      value: value,
    });
    elmessage("update output dir: " + res);
  },

  ///


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
      this.image_dir_path = selected;
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

/// init begin------------------------------
interface MsgProps {
  message: string,
  state_code: number
}

interface NotificationBackEnd {
  jpg_file_count: number
}

// 启动坚挺后端的事件。
async function backend_event_recv() {
  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  // emits the `click` event with the object payload

  const unlisten = await listen<MsgProps>("front-backend", (event) => {
    // 是一个循环函数
    // console.log(
    //   `[r]: ${event.payload.stateCode, event.payload.message}`
    // );
    // let state_code = Number(event.payload.message.substring(0, 4));
    // let data = event.payload.message.substring(4);
    switch (event.payload.state_code) {
      case 100:
        console.log(event.payload.message + "--- ");
        let result: NotificationBackEnd = JSON.parse(event.payload.message);
        image_progress.update_progress(0, result.jpg_file_count);
        break;
      case 200:
        image_progress.increase_one();
        // progress.update_progress(progress.count.completed, progress.count.total);
        break;
      case 300:
        console.log("skip file: " + event.payload.message);
        image_progress.increase_one();
        // progress.update_progress(progress.count.completed, progress.count.total);
        break;
      case 500:
        console.log("500...");
        break;
      default: console.log("unknown nofitication.: " + event.payload.message);
    }
    if (image_progress.count.completed == image_progress.count.total) {
      image_progress.status_toogle();
      elmessage(
        `[r] : 已完成处理！`
      );
    }

  });
};

// 监听 drap事件
async function drag_event_handle() {
  const unlisten = await listen<string>("tauri://file-drop", (event) => {
    // 是一个循环函数
    image_progress.dragFiles(eval(event.payload));
    console.log(event.payload);
    elmessage(
      `drap payload: ${event.payload}`
    );
  });
};



// run init function
user_conf.init_user_conf();
backend_event_recv();
drag_event_handle();


// onMounted(() => { // invalid 
// })

export { image_progress, sidebarReactives, previewwidget, elmessage, user_conf };
export type { UserDataType, UserSettings, RenameType };

