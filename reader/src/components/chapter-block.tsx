import { FC } from "react";
import Message_Block from "./message-block"
import "../styles.css";

interface chapterBlockData {
  chapter: string[][];
}

const Chapter_Block: FC<chapterBlockData> = (props) => {
  let chapter_block = [];
  let woof = "";
  let chapter: string[][] = props.chapter;
  for (let i = 0; i < chapter.length; i++) {
    if (i % 2 == 0) {
      woof = "send";
    } else {
      woof = "receive";
    }
    chapter_block.push(
      <Message_Block key={i} content={props.chapter[i]} type={woof}></Message_Block>);
  }
  return (
    chapter_block
  );
}

export default Chapter_Block;