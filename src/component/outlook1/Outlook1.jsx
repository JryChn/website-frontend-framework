import React from "react";
import "./Outlook1.module.scss";
import { handleUrl } from "../../utils/utils";
import defaultContent from "./default.json";
import style from "./Outlook1.module.scss";

export default class Outlook1 extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      content: defaultContent,
      show: [],
    };
  }
  componentDidMount() {
    this.setState({ content: handleUrl(this.props.content.address) });
    if (this.state.content.length > 3) {
      let randomNumber = Math.floor(Math.random() * this.state.content.length);
      let randomNumber1 = Math.floor(Math.random() * this.state.content.length);
      while (randomNumber1 === randomNumber) {
        randomNumber1 = Math.floor(Math.random() * this.state.content.length);
      }
      let randomNumber2 = Math.floor(Math.random() * this.state.content.length);
      while (
        randomNumber1 === randomNumber2 ||
        randomNumber === randomNumber2
      ) {
        randomNumber2 = Math.floor(Math.random() * this.state.content.length);
      }
      let showList = new Array(3);
      showList.push(this.state.content[randomNumber]);
      showList.push(this.state.content[randomNumber1]);
      showList.push(this.state.content[randomNumber2]);
      this.setState({ show: showList });
    } else {
      this.setState({ show: this.state.content });
    }
  }

  render() {
    return (
      <div
        className={style.container}
        id={this.props.content.name + this.props.index}
      >
        <section>
          <div>
            <h6>{this.props.content.name}</h6>
            <h2>{this.props.content.description}</h2>
            <a href="">Read More</a>
          </div>
        </section>
        <div>
          {this.state.show.map((content, index) => {
            return (
              <div
                id={"outlook1-content" + index}
                key={"outlook1-content" + index}
              >
                <div>
                  <a href="">
                    <img src={content.image} alt="" />
                  </a>
                </div>
                <div>
                  <h6>{content.abstract}</h6>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    );
  }
}
