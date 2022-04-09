import React from "react";
import style from "./Outlook2.module.scss";

export default class Outlook2 extends React.Component {
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
          <section>
            <div></div>
            <div>
              <span>{this.props.content.trait1}</span>
            </div>
          </section>
          <section>
            <div></div>
            <div>
              <span>{this.props.content.trait2}</span>
            </div>
          </section>
          <section>
            <div></div>
            <div>
              <span>{this.props.content.trait3}</span>
            </div>
          </section>
        </div>
        <div>
          <div>
            <img src={this.props.content.background} alt="" />
            <div>{this.props.content.who}</div>
            <div>{this.props.content.oneWords}</div>
            <div>
              <img src={this.props.protait} alt=""></img>
            </div>
            <div>{this.props.content.position}</div>
          </div>
        </div>
      </div>
    );
  }
}
