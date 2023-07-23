"use-strict";

import { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [text, setText] = useState("");
  const [chapter, setChapter] = useState(""); 

  async function load_book() {
    await invoke("beep");
    await invoke("print_from_back", { message: "hi from front" });
  }

  return (
    <div>
      <div className="container">
        <div className="menu">

        </div>
        <div className="curtain"></div>
          <div className="reader">
            <div className="test">
              <p className="send">être en rêve</p>
              <p className="receive">夢の中でる</p>
              <p className="send">i did your mom last night</p>
              <p className="receive">vorrei daverro fare a</p>
              <p className="send">我吃了你的米飯</p>
              <p className="receive">ăn nhiều bánh ngọt</p>
              <p className="send">être en rêve</p>
              <p className="receive">夢の中でる</p>
              <p className="send">i did your mom last night</p>
              <p className="receive">vorrei daverro fare a</p>
              <p className="send">我吃了你的米飯</p>
              <p className="receive">ăn nhiều bánh ngọt</p>
              <p className="send">être en rêve</p>
              <p className="receive">夢の中でる</p>
              <p className="send">i did your mom last night</p>
              <p className="receive">vorrei daverro fare a</p>
              <p className="send">我吃了你的米飯</p>
              <p className="receive">ăn nhiều bánh ngọt</p>
            </div>
        </div>
      </div>
    </div>
  );
}

export default App;
