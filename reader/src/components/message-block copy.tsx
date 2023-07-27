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
    block.push(
    <Message text={props.content[i]} type={props.type}></Message>);
  }
  return (
    block
  );
};

export default Message_Block;