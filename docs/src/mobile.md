# Mobile Apps

A main component of this project is making mobile apps as a means of delivering
most of the user experience. The app will be with people from the grocery store
to the kitchen as they compete and take part in the gamified cooking
experience.

## Tech Overview

### [flutter](https://flutter.dev/multi-platform)

Flutter will allow an iOS and Android with a single project. Dart supports ffi
for calling native code, which we can use to consume the shared Rust library
from the mobile app.

Flutter also seems to have a strong CLI rather than being bound to an IDE like
the whole Android Studio experience, which I certainly prefer.

### Rust & Dart FFI

Relevant Resources:

- [Binding to native code using dart:ffi](https://docs.flutter.dev/development/platform-integration/c-interop)
- [How to call a Rust function from Dart using FFI](https://medium.com/flutter-community/how-to-call-a-rust-function-from-dart-using-ffi-f48f3ea3af2c)
- [How to call Rust functions in Flutter (Dart) via FFI, but with convenience and safety?](https://stackoverflow.com/questions/69474386/how-to-call-rust-functions-in-flutter-dart-via-ffi-but-with-convenience-and-s)
- [fzyzcjy/flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge)
  - a project for this use case specifically
  - seems very well supported (< 1000 commits, great docs)
