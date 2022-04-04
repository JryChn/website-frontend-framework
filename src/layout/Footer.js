import { Component } from "react";
import "./Footer.css";

export default class Footer extends Component {
  render() {
    return (
      <div id="footer">
        <span id="footer-github">
            <a href={this.props.footer.github}> 
          <img src="https://img.icons8.com/dusk/64/000000/github.png"/>

                    </a>
        </span>
        <span id="footer-telegram">
            <a href={this.props.footer.telegram}>
          <img src="https://img.icons8.com/clouds/100/000000/telegram-app.png"/>
                    </a>
        </span>
      </div>
    );
  }
}
