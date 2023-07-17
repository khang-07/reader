import { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { copyFile, BaseDirectory } from '@tauri-apps/api/fs';
import "./App.css";

function App() {
  const [text, setText] = useState("");
  
  async function show_Text() {
    try {
      const path = await open({
        multiple: false,
        directory: false,
        title: "Open File"
      });

      setText(path?.toString()!);

    } catch (err) {
      console.log(err);
    }
  }

  return (
<div className="container">
      {/* <div className="menu"></div>
      <div className="reader">
        the color of this should change according to the background
      </div>
      <div className="reader">
        the color of this should change according to the background
  </div> */}
    </div>

  );
}

export default App;
