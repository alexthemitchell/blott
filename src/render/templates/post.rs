pub static TEMPLATE: &'static str = "<html>
  <head>
    <title>{{ title }} | blott</title>
  </head>
  <body>
    {{ content }}
    <h2>Join the conversation</h2>
    <a class=\"twitter-timeline\" data-dnt=\"true\" href=\"https://twitter.com/hashtag/{{ hashtag }}\" data-widget-id=\"907082457316331520\" data-query=\"{{ hashtag }}\">Tweets about \"{{ title }}\"</a>
                            <script>!function(d,s,id){var js,fjs=d.getElementsByTagName(s)[0],p=/^http:/.test(d.location)?'http':'https';if(!d.getElementById(id)){js=d.createElement(s);js.id=id;js.src=p+\"://platform.twitter.com/widgets.js\";fjs.parentNode.insertBefore(js,fjs);}}(document,\"script\",\"twitter-wjs\");</script>
  </body>
</html>";
