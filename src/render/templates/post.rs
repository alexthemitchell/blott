pub static TEMPLATE: &'static str = "<!DOCTYPE html>
<html>
  <head>
    <title>{{ title }} | blott</title>
  </head>
  <body>
    {{ content }}
    <h2>Join the conversation</h2>
    <p>Tweet with the hashtag #{{ hashtag }}</p>
    <iframe src='http://mobile.twitter.com/search?q=%23{{ hashtag }}'></iframe> 
  </body>
</html>";
