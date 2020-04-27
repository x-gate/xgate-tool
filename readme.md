# XGate Tool

The toolchain of xgate.

## TODO

### Info

- [x] 強化辨識圖片 width 及 height 合法性

### Dump

- [ ] 可匯出圖片版本 > 2 （含有調色盤）
- [x] 自動忽略空白的圖片資料
- [ ] 嘗試自動修正不合法的 width 及 height
- [ ] 修復 GraphicInfoV3（含）之後部份圖片色彩錯誤
    - 推論：可能是調色盤調用錯誤？
    - 推論：圖片全都是 3.0 之後的圖片，可能有變更過圖片生成演算法？
- [ ] 加入多執行緒
- [ ] 匯出時能出現進度條
    - 用 Channel 丟資料出來