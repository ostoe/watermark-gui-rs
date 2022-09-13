// import { image_progress } from '../scripts/reactives';
// import { onMounted, ref, reactive,unref } from 'vue';
// // 绘制进度条
// import WaveProgress from "@alanchenchen/waveprogress";
// import { drawCircle, drawText } from "@alanchenchen/waveprogress";

// const square = {
//   hook: "beforeProgress",
//   install({ ctx, configs, scopedData }, opts = {}) {
//     // ctx.beginPath();
//     // ctx.lineWidth = (opts && opts.lineWidth) || 2;
//     // ctx.strokeStyle =
//     //   (opts && opts.lineColor) || `rgba(${configs.waveCharacter.color}, 1)`;
//     // const r = Math.min(configs.canvasWidth, configs.canvasHeight) / 2;
//     // ctx.arc(r, r, r + 1, 0, 2 * Math.PI);
//     // ctx.stroke();
//     // console.log(ctx)
//     ctx.beginPath()
//     ctx.lineWidth = 5
//     ctx.strokeStyle = '#1c86d1'
//     ctx.arc(100, 200, 80, 0, Math.PI * 2)
//     ctx.stroke()
//     ctx.closePath()
//     console.log(ctx)
//   }
// };

// enum progressSettings {
//   progressSpeed = 0.2,
//   characterNum = 2,
//   characterWidth = 0.01,
//   characterHeight = 5,
//   lineWidth = 4,
//   fontSize = 10
// }

// const getProgress = (completed:number=image_progress.count.completed,total:number=image_progress.count.total) => {
//   return (completed != null
//     && total != 0
//     && total != null) ? (completed / total *100) : (0)
// }

// const waveProgressRef = ref<HTMLCanvasElement | null>(null)
// // if(canvas){
// //   const ctx = canvas!.getContext('2d')
// //   console.log(`output->ctx`, canvas)
// // }
// const waveInit=() =>{
//   const canvas = waveProgressRef.value = document.getElementById("waveProgress")!
//   // 绘制进度条
//   window.addEventListener('resize',()=>{
//     setCanvasSize(canvas!)
//   })
//   console.log(`output->canvas`,canvas)
//   return new WaveProgress({
//     canvas: canvas,
//     progress: getProgress,
//     progressSpeed: progressSettings.progressSpeed,
//     waveCharacter: {
//       number: progressSettings.characterNum,
//       waveWidth: progressSettings.characterWidth,
//       waveHeight: progressSettings.characterHeight,
//     },
//   });
// }
// const setCanvasSize = (canvas: HTMLCanvasElement) => {
//     canvas.width = 35;
//     canvas.height = 35;
// }
// const mount = onMounted(() => {
//   waveInit().usePlugin(drawCircle, { lineWidth: progressSettings.lineWidth });
//   waveInit().usePlugin(drawText, { fontSize: progressSettings.fontSize });
//   waveInit().setProgress({
//     to: getProgress
//   })
//   console.log(image_progress.count.completed)
//   console.log(waveInit());
//   waveInit().render();
// });
// watch(image_progress.count,(newValue,oldValue)=>{
//   const canvas = waveProgressRef.value = document.getElementById("waveProgress")!
//   waveInit()
//   waveInit().setProgress({
//     to: getProgress
//   })

// })
// export { square, mount,waveInit};
