import { Component } from "react";
import style from "./BlogPage.module.scss";
import defaultBlogs from "../../defaultBlogIndex.json";
import Page from "./Page";

export default class BlogPage extends Component {
  constructor(props) {
    super(props);
    this.state = {
      blogs: defaultBlogs,
    };
  }
  render() {
    return (
      <div className={style.blogPage}>
        <div>
          <ul>
            {this.state.blogs.map((item, index) => {
              return (
                <li key={"blogItems" + item.title + index}>
                  <div
                    style={{
                      backgroundImage: "url(" + item.image + ")",
                    }}
                    alt=""
                  >
                    <div>
                      <div>{item.title}</div>
                      <div>{item.abstract}</div>
                    </div>
                  </div>
                </li>
              );
            })}
          </ul>
        </div>
      </div>
    );
  }
}
