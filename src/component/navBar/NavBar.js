import { Component } from "react";
import "./NavBar.css";

export default class NavBar extends Component {
  constructor(props) {
    super(props);
    this.state = { isHumbeger: false };
    window.onresize = () => {
      this.setState({
        isHumbeger: window.innerWidth <= 800 ? true : false,
      });
    };
  }
  bar = (
    <div id="navbar">
      <ul>
        {this.props.nav.map((n, index) => {
          return (
            <li key={index} onClick={(e) => this.handleOnClick(e, index)}>
              {n}
            </li>
          );
        })}
      </ul>
      <div id="navbar-contact">
        Contact
        <div id="navbar-contact-hidden-content">
          <ul>
            <li>Telegram</li>
            <li>Email</li>
            <li>Resume</li>
            <li>MyCalendar</li>
          </ul>
        </div>
      </div>
    </div>
  );

  humbeger = (
    <div id="navbar-humbeger">
      <div>
        <div className="navbar-humbeger-border"></div>
        <div className="navbar-humbeger-border"></div>
        <div className="navbar-humbeger-border"></div>
      </div>
      <div id="navbar-humbeger-hidden-content">
        <ul>
          {this.props.nav.map((n, index) => {
            return (
              <li key={index} onClick={(e) => this.handleOnClick(e, index)}>
                {n}
              </li>
            );
          })}
          <li>Telegram</li>
          <li>Email</li>
          <li>Resume</li>
          <li>MyCalendar</li>
        </ul>
      </div>
    </div>
  );

  handleOnClick = (e, rank) => {
    let id;
    switch (rank) {
      case 0:
        id = "blog";
        break;
      case 1:
        id = "aboutme";
        break;
      case 2:
        id = "gallary";
        break;
      default:
        id = "story";
    }
    console.log(id);
    let position = document.getElementById(id).offsetTop;
    window.scrollTo({
      top: position + 200,
      behavior: "smooth",
    });
  };
  render() {
    return (
      <Naving
        isHumbeger={this.state.isHumbeger}
        bar={this.bar}
        humbeger={this.humbeger}
      ></Naving>
    );
  }
}

function Naving(props) {
  const ishumbeger = props.isHumbeger;
  if (ishumbeger) {
    return props.humbeger;
  }
  return props.bar;
}
