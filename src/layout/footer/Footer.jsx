import { Component } from "react";
import style from "./Footer.module.scss";

export default class Footer extends Component {
  render() {
    return (
      <div className={style.footer}>
        <div>
          <span>© 2018-{new Date().getFullYear()} | code by JryChn </span>
        </div>
        <div>
          <span>
            <a href={this.props.footer.github}>
              <img src="https://img.icons8.com/dusk/64/000000/github.png" />
            </a>
          </span>
          <span>
            <a href={this.props.footer.telegram}>
              <img
                src="https://img.icons8.com/clouds/100/000000/telegram-app.png"
                style={{ transform: "translateY(-20px)" }}
              />
            </a>
          </span>
        </div>
      </div>
    );
  }
}
