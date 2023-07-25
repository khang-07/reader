import { FC } from "react";
import "../styles.css";

interface msgData {
  text: string;
  type: string;
  ending?: boolean; // if end -> attach the little hook thing
}

// FC<msgData> ~ Function Component w/ parameter msgData

const Message: FC<msgData> = (props) => {
  return (
    <p className={props.type}>
      {props.text}
    </p>
  );
};

export default Message;