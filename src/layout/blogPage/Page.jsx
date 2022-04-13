import { Component, useEffect, useState } from "react";
import style from "./Page.module.scss";
import blogText1 from "../../defaultBlogText.json";
import blogText2 from "../../defaultBlogText2.json";
import blogText3 from "../../defaultBlogText3.json";
import { useParams } from "react-router-dom";

export default function Page(props) {
  let blogs = props.blog;
  const [blog, setBlog] = useState(blogText1);
  let { id } = useParams();
  useEffect(() => {
    blogs.map((eachBlog) => {
      if (eachBlog.id === parseInt(id)) {
        switch (parseInt(id)) {
          case 1:
            setBlog(blogText1);
            break;
          case 2:
            setBlog(blogText2);
            break;
          case 3:
            setBlog(blogText3);
            break;
          default:
            setBlog(blogText1);
        }
      }
    });
  });
  return (
    <div
      className={style.page}
      style={{
        backgroundImage: `url(${require("../../asset/background2.jpg")})`,
      }}
    >
      <div>
        <img src={blog.image} alt="" />
      </div>
      <div>
        <div>
          <h1>{blog.title}</h1>
          <div>
            <span> ⓦ :{blog.counts}</span>
            <span> 👀:{blog.viewCounts}</span>
            <span> 🗓:{blog.createTime}</span>
          </div>
        </div>
        <article dangerouslySetInnerHTML={{ __html: blog.content }}></article>
      </div>
    </div>
  );
}
