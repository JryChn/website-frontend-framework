import { Component } from "react";
import style from "./Outlook3.module.scss";

export default class Outlook3 extends Component {
  componentDidMount() {}

  render() {
    let list = [];
    for (let i = 0; i < 10; i++) {
      list.push(this.props.content.images[i]);
    }
    return (
      <div
        id={this.props.content.name + this.props.index}
        className={style.container}
      >
        <div>
          <div
            style={{
              fontFamily: "cursive",
              fontSize: 50,
              padding: "2% 5%",
            }}
          >
            Gallary
          </div>
        </div>
        <div>
          {list.map((url, index) => {
            return (
              <div key={"gallary" + index}>
                <img
                  src={url}
                  alt=""
                  style={{
                    width: "100%",
                    height: "100%",
                    objectFit: "cover",
                    borderRadius: "3%",
                  }}
                ></img>
              </div>
            );
          })}
        </div>
      </div>
    );
  }
}
