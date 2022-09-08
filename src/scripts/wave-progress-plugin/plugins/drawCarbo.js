/**
 * 给canvas绘制圆形边框的插件
 * 
 * @param {object} opts 通过WaveLoading实例的usePlugin方法调用的第二个参数
 * @param {number} opts.lineWidth 边框线条宽度，默认是2
 * @param {string} opts.lineColor 边框线条颜色，默认是WaveLoading实例的波浪背景色 
 */
export default {
    hook: 'duringProgress',
    install({ ctx, configs }, opts = {}) {
        ctx.beginPath()
        ctx.lineWidth = (opts && opts.lineWidth) || 2
        ctx.strokeStyle = (opts && opts.lineColor) || `rgba(${configs.waveCharacter.color}, 1)`
        let scale = ( configs.canvasHeight/ 30.0);
        let shiftx = ctx.lineWidth + 0.7 * scale;
        let shifty = - ctx.lineWidth - scale * 2.8;
        ctx.beginPath();
        ctx.moveTo(0 * scale + shiftx, 33 * scale + shifty);
        ctx.lineTo(0 * scale + shiftx, 19 * scale + shifty);
        ctx.bezierCurveTo(0 * scale + shiftx, 11 * scale + shifty, 6 * scale + shiftx, 4 * scale + shifty, 14 * scale + shiftx, 4 * scale + shifty);
        ctx.bezierCurveTo(22 * scale + shiftx, 5 * scale + shifty, 28 * scale + shiftx, 11 * scale + shifty, 28 * scale + shiftx, 19 * scale + shifty);
        ctx.lineTo(28 * scale + shiftx, 33 * scale + shifty);
        ctx.lineTo(23.333 * scale + shiftx, 28.333 * scale + shifty);
        ctx.lineTo(18.666 * scale + shiftx, 33 * scale + shifty);
        ctx.lineTo(14 * scale + shiftx, 28.333 * scale + shifty);
        ctx.lineTo(9.333 * scale + shiftx, 33 * scale + shifty);
        ctx.lineTo(4.666 * scale + shiftx, 28.333 * scale + shifty);
        ctx.lineTo(0 * scale + shiftx, 33 * scale + shifty);
        ctx.fillStyle = 'white';
        // ctx.beginPath( * scale + shifty);
        ctx.moveTo(8 * scale + shiftx, 13 * scale + shifty);
        ctx.bezierCurveTo(5 * scale + shiftx, 13 * scale + shifty, 4 * scale + shiftx, 16 * scale + shifty, 4 * scale + shiftx, 18 * scale + shifty);
        ctx.bezierCurveTo(4 * scale + shiftx, 20 * scale + shifty, 5 * scale + shiftx, 23 * scale + shifty, 8 * scale + shiftx, 23 * scale + shifty);
        ctx.bezierCurveTo(11 * scale + shiftx, 23 * scale + shifty, 12 * scale + shiftx, 20 * scale + shifty, 12 * scale + shiftx, 18 * scale + shifty);
        ctx.bezierCurveTo(12 * scale + shiftx, 16 * scale + shifty, 11 * scale + shiftx, 13 * scale + shifty, 8 * scale + shiftx, 13 * scale + shifty);
        ctx.moveTo(20 * scale + shiftx, 13 * scale + shifty);
        ctx.bezierCurveTo(17 * scale + shiftx, 13 * scale + shifty, 16 * scale + shiftx, 16 * scale + shifty, 16 * scale + shiftx, 18 * scale + shifty);
        ctx.bezierCurveTo(16 * scale + shiftx, 20 * scale + shifty, 17 * scale + shiftx, 23 * scale + shifty, 20 * scale + shiftx, 23 * scale + shifty);
        ctx.bezierCurveTo(23 * scale + shiftx, 23 * scale + shifty, 24 * scale + shiftx, 20 * scale + shifty, 24 * scale + shiftx, 18 * scale + shifty);
        ctx.bezierCurveTo(24 * scale + shiftx, 16 * scale + shifty, 23 * scale + shiftx, 13 * scale + shifty, 20 * scale + shiftx, 13 * scale + shifty);
        // ctx.fill();
        ctx.clip();
        ctx.stroke();
    }
}