import { FC } from "react";
import Message from "./message"
import "../styles.css";

interface msgBlockData {
  content: string[];
  type: string; // send or receive
}

// FC<msgData> ~ Function Component w/ parameter msgData

const Message_Block: FC<msgBlockData> = (props) => {
  let block = [];
  for (let i = 0; i < props.content.length; i++) {
    let moi = (i == props.content.length - 1) ? true : false;
    block.push(
      <Message key={i} text={props.content[i]} type={props.type} ending={moi}></Message>);
  }
  return (
    block
  );
};

export default Message_Block;