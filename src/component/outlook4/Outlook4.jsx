import React from "react";
import style from "./Outlook4.module.scss";

export default class Outlook4 extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      storys: [
        { img: "https://source.unsplash.com/random/?city,night" },
        { img: "https://source.unsplash.com/random/?fruit" },
        { img: "https://source.unsplash.com/random/?city,night" },
        { img: "https://source.unsplash.com/random/?city,night" },
        { img: "https://source.unsplash.com/random/?city,night" },
        { img: "https://source.unsplash.com/random/?city,night" },
      ],
    };
  }
  render() {
    return (
      <div
        id={this.props.content.name + this.props.index}
        className={style.container}
      >
        <div>
          <div>
            <h6>{this.props.content.name}</h6>
            <h2>{this.props.content.description}</h2>
          </div>
        </div>
        <div>
          <div>
            {this.state.storys.map((story, index) => {
              return (
                <div className={style.content} key={"story" + index}>
                  <img src={story.img} alt="" />
                </div>
              );
            })}
          </div>
        </div>
      </div>
    );
  }
}
