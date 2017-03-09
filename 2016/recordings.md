---
layout: default-2016
title: Recordings - !!Con
---

# Talk Recordings

<section id="talk_container"></section>

<div id="talk-template" style="display:none" class="talk">
  <h3 class="talk-info"></h3>
  <div class="talk-youtube-thumb"></div>
  <div class="talk-youtube"></div>
  <div class="talk-embed"></div>
  <div class="talk-transcript"></div>
  <div style="clear:both"></div>
</div>

<script src="//ajax.googleapis.com/ajax/libs/jquery/1.11.1/jquery.min.js"></script>
<script type="text/javascript" src="../js/recordings.js"></script>
<script defer="defer">
  jQuery.getJSON('talks.json', function(talks) {
    generateTalks(
      '#talk-template',
      '#talk_container',
      talks,
      "../2016-transcripts/"
    );
  });
</script>
