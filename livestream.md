---
layout: default-2021
title: Livestream - !!Con 2021
---

<script>
let allSessions = [
{
    name: "Opening Keynote + Session 1 Live",
    startDateTime: "Sat May 15 2021 14:55:00 GMT-0400",
    endDateTime:   "Sat May 15 2021 16:59:59 GMT-0400",
    link: "https://www.youtube.com/embed/Umv3jPS8b7E"
}, {
    name: "Session 2 Live",
    startDateTime: "Sat May 15 2021 18:00:00 GMT-0400",
    endDateTime:   "Sat May 15 2021 18:59:59 GMT-0400",
    link: "https://www.youtube.com/embed/V51nIE6cnkI"
}, {
    name: "Session 1 Syncwatch",
    startDateTime: "Sun May 16 2021 13:00:00 GMT-0400",
    endDateTime:   "Sun May 16 2021 14:59:59 GMT-0400",
    link: "https://www.youtube.com/embed/lH6NYVIasi0"
}, {
    name: "Session 2 Syncwatch",
    startDateTime: "Sun May 16 2021 15:00:00 GMT-0400",
    endDateTime:   "Sun May 16 2021 15:59:59 GMT-0400",
    link: "https://www.youtube.com/embed/77HNl3ZsU-E"
}, {
    name: "Session 3 Live",
    startDateTime: "Sun May 16 2021 20:00:00 GMT-0400",
    endDateTime:   "Sun May 16 2021 20:59:59 GMT-0400",
    link: "https://www.youtube.com/embed/g_eMNX4OvoY"
}];
</script>

<p style="text-align: center;">
  **Quick Links:**
  [Speakers](speakers.html)
  &middot;
  [Program](program.html)
  &middot;
  [Conduct](conduct.html)
</p>

_**Many thanks to [Wherewithall](https://courses.wherewithall.com/pages/bangbangcon) for sponsoring the captioning for this year's !!Con!**_

Select the session to watch: 
<select id="sessionDropDown">
<option id="defaultSession">Present Session</option>
</select> <button type="button" id="updateVideoButton">Update Video</button>

<div align="center">
<style>.embed-container { position: relative; padding-bottom: 56.25%; height: 0; overflow: hidden; max-width: 100%; } .embed-container iframe, .embed-container object, .embed-container embed { position: absolute; top: 0; left: 0; width: 100%; height: 100%; }</style><div class='embed-container'>
<iframe id="youtubeIframe" src='https://www.youtube.com/embed/V51nIE6cnkI' frameborder='0' allowfullscreen>
</iframe></div>
</div>


<h3> live captioning of talks </h3>
<div align="center">
<style>.embed-container { position: relative; padding-bottom: 56.25%; height: 0; overflow: hidden; max-width: 100%; } .embed-container iframe, .embed-container object, .embed-container embed { position: absolute; top: 0; left: 0; width: 100%; height: 100%; }</style><div class='embed-container'><iframe src='https://www.streamtext.net/player?event=bangbangcon' frameborder='0' allowfullscreen></iframe></div>
<a href="https://www.streamtext.net/player?event=bangbangcon">or open in separate window </a>
</div>
<br><br>

For updates on !!Con, follow
[@bangbangcon](https://twitter.com/bangbangcon) on Twitter, or sign up
for our mailing list below.  We send 4-5 emails per year about our venue, submission deadlines, registration, etc.

<!-- Begin MailChimp Signup Form -->
<div id="mc_embed_signup">
<form action="http://bangbangcon.us3.list-manage.com/subscribe/post?u=37b924b9d7d71dc7aa1a52b4c&amp;id=9f9ec7c469" method="post" id="mc-embedded-subscribe-form" name="mc-embedded-subscribe-form" class="validate" target="_blank" style="background-color: inherit;" novalidate>
<div class="mc-field-group">
<label for="mce-EMAIL">Email:</label>
<input type="email" value="" name="EMAIL" class="required email" id="mce-EMAIL" placeholder='your email address'>
<input type="submit" value="Subscribe" name="subscribe" id="mc-embedded-subscribe" class="button">
</div>
<div id="mce-responses" class="clear">
<div class="response" id="mce-error-response" style="display:none"></div>
<div class="response" id="mce-success-response" style="display:none"></div>
</div>
<!-- real people should not fill this in and expect good things - do not remove this or risk form bot signups-->
<div style="position: absolute; left: -50020px;">
<input type="text" name="b_37b924b9d7d71dc7aa1a52b4c_9f9ec7c469" value="">
</div>
</form>
</div>

<script>
// Delete the default session from the drop down box
document.getElementById("defaultSession").remove();

// Populate drop down with all sessions
let dropDownBox = document.getElementById("sessionDropDown");
let nextSession = -1;
allSessions.forEach((session, index) => {
    // Build option
    let option = document.createElement("option");
    option.value = index;
    option.innerHTML = session.name;
    option.selected = false;

    // Determine if it should be selected (the next session coming up)
    let startDate = new Date(session.startDateTime);
    let endDate = new Date(session.endDateTime);
    let now = new Date();

    // If it's now in the middle of a session, mark it "LIVE" and select it
    if (now >= startDate && now <= endDate) {
        option.innerHTML = "LIVE: " + option.innerHTML;
        nextSession = index;
        option.selected = true;
    }
    // If it's not in a session, find the next one, mark it "NEXT" and select it
    else if (nextSession === -1 && now <= endDate) {
        option.innerHTML = "NEXT: " + option.innerHTML;        
        nextSession = index;
        option.selected = true;
    }

    // Add option to dropdown list
    dropDownBox.appendChild(option);
});

// If it's over, there's no live or next session. Just load first one.
if (nextSession === -1) nextSession = 0;

// Load the right Youtube link into the iFrame
let iFrame = document.getElementById("youtubeIframe");
iFrame.src = allSessions[nextSession].link;

document.getElementById("updateVideoButton").addEventListener("click", updateVideoLink);

// Ran when user presses Update Video button
function updateVideoLink() {
    let selectedIndex = dropDownBox.selectedIndex;
    iFrame.src = allSessions[selectedIndex].link;
}

</script>
