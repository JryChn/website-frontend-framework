import { Component } from "react";
import Introduce from "../component/introduce/Introduce";

export default class MainContent extends Component {
  render() {
    return <Introduce introduce={this.props.introduce}></Introduce>;
  }
}
