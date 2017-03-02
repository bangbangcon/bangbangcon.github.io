;(function(exports) {
  var generateTemplate = function(talk, template) {
    var newTalk = $(template).clone();
    newTalk.attr({
      id: ''
    });
    newTalk.find('.talk-info').html('<a href="./speakers.html#' + talk.authorslug + '">' + talk.author + '</a><strong><em>' + talk.title + '</em></strong>');
    if (talk.youtube) {
      newTalk.find('.talk-youtube-thumb').html('<a href="http://youtube.com/watch?v=' + talk.youtube + '"><img src="http://img.youtube.com/vi/' + talk.youtube + '/1.jpg" alt="" /></a>');
      newTalk.find('.talk-youtube').html('<a href="http://youtube.com/watch?v=' + talk.youtube + '">View on YouTube!</a>');
      newTalk.find('.talk-embed').html('<a>View right here!</a>').click(function(evt) {
        embed($(evt.target).parents('.talk'), talk.youtube)
      });
    } else {
      newTalk.find('.talk-youtube').text('There is no video or transcript for this talk.');
    }
    if (talk.transcript) {
      newTalk.find('.talk-transcript').html('<a href="/2015-transcripts/' + talk.authorslug + '.txt">Read the transcript!</a>');
    }
    newTalk.show();
    return newTalk;
  }

  exports.generateTalks = function(template, appendTo, talks) {
    appendTo = $(appendTo);
    template = $(template);
    for (var i = 0; i < talks.length; i++) {
      appendTo.append(generateTemplate(talks[i], template));
    }
  }

  exports.embed = function(target, youtube) {
    $('#youtube').remove();
    var iframe = '<iframe id="youtube" width="640" height="360" src="//www.youtube.com/embed/' + youtube + '?rel=0" frameborder="0" allowfullscreen></iframe>'
    target.append(iframe);
  }

})(window);

$(document).ready(function(){
  jQuery.getJSON('talks.json', function(talks) {
    generateTalks('#talk-template', '#talk_container', talks);
  });
});
