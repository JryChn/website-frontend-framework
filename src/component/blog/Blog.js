import React from "react";
import "./Blog.css";
import { handleUrl } from "../../utils/utils";

export default class Blog extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      blogs: [
        {
          id: 1,
          title: "Hello world",
          abstract: " this is a test abstract",
          time: "2016-11-01",
          content: "this is also a test content",
          image: "https://source.unsplash.com/1600x900/?beach",
        },
        {
          id: 2,
          title: "Bye world",
          abstract: " this is a test  oho abstract",
          time: "2016-11-01",
          content: "this is also a test content",
          image: "https://source.unsplash.com/1600x900/?beach",
        },
        {
          id: 3,
          title: "Missing world",
          abstract: " this is a test test bstract",
          time: "2016-11-01",
          content: "this is also a test content",
          image: "https://source.unsplash.com/1600x900/?beach",
        },
        {
          id: 4,
          title: "Hello world1",
          abstract: " 1this is a test abstract",
          time: "2016-11-01",
          content: "this is also a test content",
          image: "https://source.unsplash.com/1600x900/?beach",
        },
        {
          id: 5,
          title: "Bye world2",
          abstract: " 2this is a test  oho abstract",
          time: "2016-11-01",
          content: "this is also a test content",
          image: "https://source.unsplash.com/1600x900/?beach",
        },
        {
          id: 6,
          title: "33Missing world",
          abstract: " this dfdsfsdfis a test test bstract",
          time: "2016-11-01",
          content: "this is also a test content",
          image: "https://source.unsplash.com/1600x900/?beach",
        },
      ],
      show: [],
    };
  }
  componentDidMount() {
    this.setState({ blogs: handleUrl(this.props.blog.address) });
    if (this.state.blogs.length > 3) {
      let randomNumber = Math.floor(Math.random() * this.state.blogs.length);
      let randomNumber1 = Math.floor(Math.random() * this.state.blogs.length);
      while (randomNumber1 === randomNumber) {
        randomNumber1 = Math.floor(Math.random() * this.state.blogs.length);
      }
      let randomNumber2 = Math.floor(Math.random() * this.state.blogs.length);
      while (
        randomNumber1 === randomNumber2 ||
        randomNumber === randomNumber2
      ) {
        randomNumber2 = Math.floor(Math.random() * this.state.blogs.length);
      }
      let showList = new Array(3);
      showList.push(this.state.blogs[randomNumber]);
      showList.push(this.state.blogs[randomNumber1]);
      showList.push(this.state.blogs[randomNumber2]);
      this.setState({ show: showList });
    } else {
      this.setState({ show: this.state.blogs });
    }
  }

  render() {
    return (
      <div className="blog" id="blog">
        <section id="blog-words-container">
          <div id="blog-words">
            <h6>{this.props.blog.name}</h6>
            <h2>{this.props.blog.description}</h2>
            <a href="">Read More</a>
          </div>
        </section>
        <div id="blog-container">
          {this.state.show.map((blog, index) => {
            return (
              <div id={"blog" + index} key={"blog" + index}>
                <div>
                  <a href="">
                    <img src={blog.image} alt="" />
                  </a>
                </div>
                <div>
                  <h6>{blog.abstract}</h6>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    );
  }
}
