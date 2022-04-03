import { Component } from "react";
import Header from "./layout/Header";
import MainContent from "./layout/MainContent";
// import Footer from "./layout/Footer";
import config from "./config.json";

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      config: config,
    };
  }

  render() {
    return (
      <div>
        <Header
          web_url={this.state.config.url}
          logo={this.state.config.logo}
          header={this.state.config.section}
        ></Header>
        <MainContent
          introduce={this.state.config.introduce}
          main={this.state.config.section}
        ></MainContent>
      </div>
    );
  }
}
