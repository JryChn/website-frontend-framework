import { Component } from "react";
import style from "./Outlook3.module.scss";

export default class Outlook3 extends Component {
  componentDidMount() {
    document.getElementById("gallaryComponent").style.display = "grid";
    document.getElementById("gallaryComponent").style.gridTemplateRows =
      "1fr 1fr 1fr 1fr ";

    document.getElementById("gallaryComponent").style.gridTemplateColumns =
      "1fr 1fr 1fr 1fr 1fr 1fr";
    // document.getElementById("gallary1").style.gridRow = "1/3";
    // document.getElementById("gallary1").style.gridColumn = "1/3";

    // document.getElementById("gallary2").style.gridRow = "3/6";
    // document.getElementById("gallary2").style.gridColumn = "1/3";

    // document.getElementById("gallary3").style.gridRow = "1/2";
    // document.getElementById("gallary3").style.gridColumn = "3/4";

    // document.getElementById("gallary4").style.gridRow = "1/2";
    // document.getElementById("gallary4").style.gridColumn = "4/6";

    // document.getElementById("gallary5").style.gridRow = "2/3";
    // document.getElementById("gallary5").style.gridColumn = "4/6";

    // document.getElementById("gallary6").style.gridRow = "3/5";
    // document.getElementById("gallary6").style.gridColumn = "3/4";

    // document.getElementById("gallary7").style.gridRow = "5/6";
    // document.getElementById("gallary7").style.gridColumn = "3/6";

    // document.getElementById("gallary8").style.gridRow = "3/4";
    // document.getElementById("gallary8").style.gridColumn = "4/6";

    // document.getElementById("gallary9").style.gridRow = "3/5";
    // document.getElementById("gallary9").style.gridColumn = "4/6";

    // document.getElementById("gallary10").style.gridRow = "1/3";
    // document.getElementById("gallary10").style.gridColumn = "1/3";
  }

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
        <div id="gallaryComponent">
          {list.map((url, index) => {
            return (
              <div key={"gallary" + index} id={"gallary" + index}>
                <img
                  src={url}
                  alt=""
                  style={{
                    width: "99%",
                    height: "99%",
                    objectFit: "",
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
