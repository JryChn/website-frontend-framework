import React from "react";
import style from "./Outlook2.module.scss";

export default class Outlook2 extends React.Component {
  render() {
    return (
      <div
        id={this.props.content.name + this.props.index}
        className={style.container}
        style={{
          // backgroundImage: `url(${require("../../asset/background3.jpg")})`,
          backgroundImage: `url("` + this.props.content.background + `")`,
          backgroundAttachment: "fixed",
        }}
      >
        <div>
          <div>
            <h6>{this.props.content.name}</h6>
            <h2>{this.props.content.description}</h2>
          </div>
        </div>
        <div>
          <section>
            <div>
              <img
                src="https://img.icons8.com/external-parzival-1997-flat-parzival-1997/64/000000/external-passion-love-story-parzival-1997-flat-parzival-1997.png"
                alt=""
              />
            </div>
            <div>
              <span>{this.props.content.trait1}</span>
            </div>
          </section>
          <section>
            <div>
              <img
                src="https://img.icons8.com/ios-filled/100/000000/cat-profile.png"
                alt=""
              />
            </div>
            <div>
              <span>{this.props.content.trait2}</span>
            </div>
          </section>
          <section>
            <div>
              <img
                src="https://img.icons8.com/external-others-pike-picture/100/000000/external-paranoia-problems-others-pike-picture.png"
                alt=""
              />
            </div>
            <div>
              <span>{this.props.content.trait3}</span>
            </div>
          </section>
        </div>
        <div>
          <div>
            <img src={this.props.content.protraitBackground} alt="" />
            <div>{this.props.content.who}</div>
            <div>{this.props.content.oneWords}</div>
            <div>
              <img src={this.props.content.protait} alt=""></img>
            </div>
            <div>{this.props.content.position}</div>
          </div>
        </div>
      </div>
    );
  }
}
