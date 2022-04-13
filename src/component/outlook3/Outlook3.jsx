import { Component } from "react";
import style from "./Outlook3.module.scss";

export default class Outlook3 extends Component {
  constructor(props) {
    super(props);
    this.state = {
      list: [],
    };
  }
  componentDidMount() {
    let list = [];
    for (let i = 0; i < 10; i++) {
      list.push(this.props.content.images[i]);
    }
    while (list.length < 10) {
      list.push(this.props.content.images[0]);
    }
    this.setState({ list: list });
  }
  render() {
    return (
      <div
        id={this.props.content.name + this.props.index}
        className={style.container}
        style={{
          backgroundImage: `url(${require("../../asset/background4.jpg")})`,
          backgroundSize: "cover",
        }}
      >
        <div>
          <div style={{}}>Gallary</div>
        </div>
        <div>
          {this.state.list.map((image, index) => {
            return (
              <figure
                key={"gallary" + index}
                style={{
                  gridArea: String.fromCharCode(parseInt(index) + 97),
                }}
              >
                <img src={image.url} alt="" />
                <figcaption>{image.description}</figcaption>
              </figure>
            );
          })}
        </div>
      </div>
    );
  }
}
