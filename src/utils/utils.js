function handleUrl(url) {
  let regex = /^http(s)?:\/\/([\w-]+\.)+[\w-]+(\/[\w- ./?%&=]*)?$/;
  if (regex.test(url)) {
    return fetch(url);
  }
  return url;
}

export { handleUrl };
