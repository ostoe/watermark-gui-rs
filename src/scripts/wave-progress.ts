const square = {
  hook: "beforeProgress",
  install({ ctx, configs, scopedData }, opts = {}) {
    // ctx.beginPath();
    // ctx.lineWidth = (opts && opts.lineWidth) || 2;
    // ctx.strokeStyle =
    //   (opts && opts.lineColor) || `rgba(${configs.waveCharacter.color}, 1)`;
    // const r = Math.min(configs.canvasWidth, configs.canvasHeight) / 2;
    // ctx.arc(r, r, r + 1, 0, 2 * Math.PI);
    // ctx.stroke();
    // console.log(ctx)
    ctx.beginPath()
    ctx.lineWidth = 5
    ctx.strokeStyle = '#1c86d1'
    ctx.arc(100, 200, 80, 0, Math.PI * 2)
    ctx.stroke()
    ctx.closePath()
    console.log(ctx)
  }
};

export { square };
