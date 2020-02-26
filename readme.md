# XGate Tool

The toolchain of xgate.

## TODO

### Info

[ ] 強化辨識圖片 width 及 height 合法性

> Graphic_PUK2_2.bin 編號 1948：寬度 4、高度 4294967270、大小 16，理論上高度推斷應該是 4

### Dump

[ ] 可匯出圖片版本 > 2 （含有調色盤）
[ ] 嘗試自動修正不合法的 width 及 height
[ ] 修復 GraphicInfoV3（含）之後部份圖片色彩錯誤
    - 推論：可能是調色盤調用錯誤？
    - 推論：圖片全都是 3.0 之後的圖片，可能有變更過圖片生成演算法？
[ ] 加入多執行緒
[ ] 匯出時能出現進度條
    - 用 Channel 丟資料出來