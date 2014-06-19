;(function(exports) {

	var talks = [
		{
			author: "Michael Bernstein",
			authorslug: "mbernstein",
			title: "The Art of Obsession",
			youtube: "yJgwrk6_zGY",
			transcript: "michael-bernstein-the-art-of-obsession",
		},
		{
			author: "Travis McDemus",
			authorslug: "tmcdemus",
			title: "The Sound of Segfaults!!",
			youtube: "XnsnHS-YGXM",
			transcript: "travis-mcdemus-the-sound-of-segfaults",
		},
		{
			author: "Darius Bacon and Danielle Sucher",
			authorslug: "dbacondsucher",
			title: "Nantucket! Hacking at verse",
			youtube: "Rn97kZNZ278",
			transcript: "danielle-sucher-darius-bacon-nantucket-hacking-at-verse",
		},
		{
			author: "David Turner",
			authorslug: "dturner",
			title: "Now you're thinking with PCMPISTRI!",
			youtube: "U3gsPgryfKs",
			transcript: "david-turner-now-youre-thinking-with-pcmpistri",
		},
		{
			author: "Bjorn Roche",
			authorslug: "broche",
			title: "How I used my knowledge of code (and music!) to help fight fires!",
			youtube: "jHu9cCijCZc",
			transcript: "bjorn-roche-how-i-used-knowledge-of-code-and-music-to-fight-forest-fires",
		},
		{
			author: "Chloe Weil",
			authorslug: "cweil",
			title: "Tasty Stacks!",
			youtube: "8218rXTggWk",
			transcript: "chloe-weil-tasty-stacks",
		},
		{
			author: "Julia Evans",
			authorslug: "jevans",
			title: "Spying on your programs with strace!!!",
			youtube: "4pEHfGKB-OE",
			transcript: "julia-evans-spying-on-your-programs-with-strace",
		},
		{
			author: "Jennifer Shin",
			authorslug: "jshin",
			title: "High Schoolers vs. Robots!!!!",
			youtube: "mZ1ZRAxUjfY",
			transcript: "jennifer-shin-high-schoolers-versus-robots",
		},
		{
			author: "Kamelia Aryafar",
			authorslug: "karyafar",
			title: "Computer vision and archeology!: Can computers assemble ceramic artifacts?",
			youtube: false, // :(
			transcript: "kamelia-aryafar-computer-vision-archaeology",
		},
		{
			author: "Allison Kaptur",
			authorslug: "akaptur",
			title: "A 1,500 line (!!) switch statement powers your Python!",
			youtube: "4s9MkZATWY4",
			transcript: "allison-kaptur-switch-statement-1500-lines",
		},
		{
			author: "Aki Yamada",
			authorslug: "ayamada",
			title: "Just LOOK at the humongous type that Hindley-Milner infers for this tiny program!",
			youtube: "PoWtBY-Ex1A",
			transcript: "aki-yamada-hindley-milner",
		},
		{
			author: "Michael Arntzenius",
			authorslug: "marntzenius",
			title: "Continuations; or, how to travel through time!",
			youtube: "cnhb4M8-J5M",
			transcript: "michael-arntzenius-continuations",
		},
		{
			author: "Allie Jones",
			authorslug: "ajones",
			title: "Weaving and Programming: More Related Than You (Probably) Realize!",
			youtube: "8TfKeoxtq2c",
			transcript: "allie-jones-weaving-programming",
		},
		{
			author: "Camille Fournier",
			authorslug: "cfournier",
			title: "How to Stay in Love with Programming",
			youtube: "sc8sc-ELMhA",
			transcript: "camille-fournier-how-to-stay-in-love-with-programming",
		},
		{
			author: "Lisa Neigut",
			authorslug: "lneigut",
			title: "Serial! It's what's for breakfast",
			youtube: "J1CQz8XyWoo",
			transcript: "lisa-neigut-serial-its-whats-for-breakfast",
		},
		{
			author: "Andrew Gwozdziewycz",
			authorslug: "agwozdziewycz",
			title: "Understanding Garbage Collection, through Visualizing a One Pass Real-Time Generational Mark-Sweep Garbage Collector!",
			youtube: "QLzhB6c8uCg",
			transcript: "andrew-gwozdziewycz-understanding-garbage-collection",
		},
		{
			author: "Emily Reese",
			authorslug: "ereese",
			title: "L'Artiste et Le Programmeur!",
			youtube: "RQjxVAhzUzg",
			transcript: "emily-reese-lartiste-et-le-programmeur",
		},
		{
			author: "Rafik Draoui",
			authorslug: "rdraoui",
			title: "Making music with floppy drives! An exercise in yak shaving",
			youtube: "Hbdhh0fWbfM",
			transcript: "rafik-draoui-making-music-with-floppy-drives",
		},
		{
			author: "Guillaume Marceau",
			authorslug: "gmarceau",
			title: "The terrible Yook monster! Slayed by the grandson of Master Prolog!",
			youtube: "Q663XyxmGUk",
			transcript: "guillaume-marceau-the-terrible-yook-monster",
		},
		{
			author: "Mark Wunsch",
			authorslug: "mwunsch",
			title: "Map, Reduce, Awk!",
			youtube: "jw-3Ufd_u4c",
			transcript: "mark-wunsch-map-reduce-awk",
		},
		{
			author: "Paul Khuong",
			authorslug: "pkhuong",
			title: "LZ77 refactors program traces!",
			youtube: "Z-Aeg-9WcMQ",
			transcript: "paul-khuong-lz77-refactors-program-traces",
		},
		{
			author: "Omar Rizwan",
			authorslug: "orizwan",
			title: "We found chat in a hostile place!!",
			youtube: "UBop5kbqDMo",
			transcript: "omar-rizwan-we-found-chat-in-a-hostile-place",
		},
		{
			author: "Katherine Ye",
			authorslug: "kye",
			title: "Proofs about programs, proofs as programs, and programs as proofs!",
			youtube: "ghIGfwmosSc",
			transcript: "katherine-ye-proofs-about-programs-proofs-as-programs",
		},
		{
			author: "Daniel Luxemberg",
			authorslug: "dluxemberg",
			title: "Brainwaves! On your computer!",
			youtube: "B4EO4gei4fc",
			transcript: "daniel-luxemburg-brainwaves-on-your-computer",
		},
		{
			author: "Adam Parrish",
			authorslug: "aparrish",
			title: "Scrabble sucks! Toward higher-order word games",
			youtube: "N9FRaxbiR_8",
			transcript: "adam-parrish-scrabble-sucks-toward-higher-order-word-games",
		},
		{
			author: "Mark Jason Dominus",
			authorslug: "mjdominus",
			title: "Help! Help!",
			youtube: "uvcd5sIw96U",
			transcript: "mark-jason-dominus-help-help",
		}
	];

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
			newTalk.find('.talk-youtube').text('Unfortunately, the recording of this talk was lost due to a technical issue.');
		}
		newTalk.find('.talk-transcript').html('<a href="./2014-transcripts/' + talk.transcript + '.txt">Read the transcript!</a>');
		newTalk.show();
		return newTalk;
	}

	exports.generateTalks = function(template, appendTo) {
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
	generateTalks('#talk-template', '#talk_container');
});
