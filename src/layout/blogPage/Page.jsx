import { Component } from "react";
import style from "./Page.module.scss";
import blogText from "../../defaultBlogText.json";

export default class Page extends Component {
  constructor(props) {
    super(props);
    this.state = {
      blog: blogText,
    };
  }
  render() {
    return (
      <div className={style.page}>
        <div>
          <img src={this.state.blog.image} alt="" />
        </div>
        <div>
          <div>
            <h1>{this.state.blog.title}</h1>
            <div>
              <span> counts:{this.state.blog.counts}</span>
              <span> view counts:{this.state.blog.viewCounts}</span>
              <span> createTime:{this.state.blog.createTime}</span>
            </div>
          </div>
          <article
            dangerouslySetInnerHTML={{ __html: this.state.blog.content }}
          ></article>
        </div>
      </div>
    );
  }
}
