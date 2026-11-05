# 悬停展开尺寸试验

基于上游 v0.6.50，将展开面板、文字、按钮和输入框放大为 1.5 倍，收起状态保持原尺寸。

- 普通页面主体：628.5 × 210 逻辑单位。
- 日历页面主体：628.5 × 324 逻辑单位。
- 岛内邮件正文：628.5 × 354 逻辑单位。
- 保持顶部中心、悬停意图判断和移开延迟；鼠标命中及文件拖入区域与绘制尺寸同步。
- 扩大透明宿主画布，容纳放大后的轮廓与阴影；设置、独立邮件阅读窗口保持原尺寸。

自动检查覆盖原收起尺寸与字号、五个功能页、多倍率 DPI、展开后新增区域悬停、移开收起和中途反转、待办及日程输入框、日期点击与音乐进度定位。测试使用独立临时目录与合成内容，不连接私人账号。

```powershell
cargo test --locked --all-targets --features record -- --test-threads=1
$env:DAOZAI_PREVIEW_DIR = 'target/ui-previews'
cargo test --locked --lib --features record enlarged_panel_preview_gallery -- --ignored --test-threads=1
```

旧回归测试及旧录制脚本明确采用 1.0 倍设计坐标；新增回归检查使用程序默认的 1.5 倍展开尺寸。Windows CI 会保存测试版安装包、校验文件和原尺寸／放大截图。
