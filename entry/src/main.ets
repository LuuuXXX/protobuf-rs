// Entry point for HarmonyOS application
// This is a minimal entry module for protobuf-rs HarmonyOS integration

@Entry
@Component
struct Index {
  @State message: string = 'protobuf-rs HarmonyOS'

  build() {
    Row() {
      Column() {
        Text(this.message)
          .fontSize(50)
          .fontWeight(FontWeight.Bold)
      }
      .width('100%')
    }
    .height('100%')
  }
}
