import { FC } from "react";
import Chapter_Block from "./chapter-block";
import "../styles.css";

interface readerData {
  chapter: string[][];
}

// FC<msgData> ~ Function Component w/ parameter msgData

const Message: FC<readerData> = (props) => {
  return (
    <div className="reader">
    <div className="message-wrapper">
        <Chapter_Block chapter={props.chapter}></Chapter_Block>
    </div>
  </div>
  );
};

export default Message;