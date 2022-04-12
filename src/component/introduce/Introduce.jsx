import { Component } from "react";
import style from "./Introduce.module.scss";

export default class Introduce extends Component {
  render() {
    return (
      <div
        className={style.container}
        id="introduce"
        style={{
          backgroundImage: `url(${require("../../asset/background.jpg")})`,
        }}
      >
        <div>
          <h4>{this.props.introduce.subTitle}</h4>
          <h1>{document.title + ".com"}</h1>
          <h6>{this.props.introduce.words}</h6>
        </div>
        <div>
          <img src={this.props.introduce.mainImage} alt=""></img>
        </div>
        <div>
          <h6>{'"' + this.props.introduce.quote.content + '"'}</h6>
          <h6>{"- " + this.props.introduce.quote.celebrity}</h6>
        </div>
      </div>
    );
  }
}
