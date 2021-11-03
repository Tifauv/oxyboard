// Used as a global lock to prevent several simultaneous updates to the
// board DOM.
var g_updating = false;


function showSuccessFeedback(p_message) {
	addAlert("success", p_message);
	setTimeout(closeAlert, 2000);
}

function showWarningFeedback(p_message) {
	addAlert("warning", p_message);
	setTimeout(closeAlert, 8000);
}

function addAlert(p_type, p_message) {
	$("<div/>", {
		id     : "feedback-alert",
		"class": "alert alert-dismissible alert-" + p_type,
		role   : "alert",
		html   : "<button type=\"button\" class=\"close\" data-dismiss=\"alert\" aria-label=\"Close\"><span aria-hidden=\"true\">&times;</span></button>" + p_message
	}).appendTo("#feedback");
}

function closeAlert() {
	$("#feedback-alert").alert("close");
}

function postToHtml(p_post) {
	var id         = p_post.attr("id"),
		time       = p_post.attr("time"),
		date       = time.substring(6,8) + "/" + time.substring(4,6) + "/" + time.substring(0,4),
		clock      = time.substring(8,10) + ":" + time.substring(10,12) + ":" + time.substring(12),
		user_agent = p_post.find("info").text(),
		login      = p_post.find("login").text(),
		author     = (login.length == 0 ? user_agent.substring(0, 16) : login),
		message    = p_post.find("message").text();
	return "<tr id=\"post-" + id + "\">"
		+ "<td class=\"post-author\" title=\"" + user_agent + "\">" + author + "</td>"
		+ "<td class=\"post-time\" title=\"" + date + "\">" + clock + "</td>"
		+ "<td class=\"post-message text-justify\">" + message + "</td>"
		+ "</tr>";
}

function insertAtCaret(p_element, p_text) {
	var caretPos = p_element[0].selectionStart,
		currentValue = p_element.val();

	p_element.val(currentValue.substring(0, caretPos) + p_text + currentValue.substring(caretPos));
	p_element.focus();
}


function loadBoard() {
	$.ajax({
		url     : "/backend/last/100",
		dataType: "xml"
	}).done(function(p_response) {
		var newPostsHtml = "";
		$(p_response).find("post")
			.nextAll()
			.each(function() {
				newPostsHtml = postToHtml($(this)) + newPostsHtml;
			});

		// Fill the board display with the build content
		if (newPostsHtml.length > 0) {
			$("#board table").append(newPostsHtml);
		}

		// Add a click event handler to the authors
		$("#board").on("click", ".post-author", function() {
			console.log("Selected author " + $(this).text());
			insertAtCaret( $("#message"), $(this).text() + "< ");
		});

		// Add a click event handler to the clocks
		$("#board").on("click", ".post-time", function() {
			console.log("Selected clock " + $(this).text());
			insertAtCaret( $("#message"), $(this).text() + " ");
		});

	}).fail(function(p_response, p_status, p_message) {
		console.log("Backend retrieval failed");
		console.log("  message: " + p_message);
		console.log("   status: " + p_status);
	});

	$("#message").val("");
	$("#message").focus();
}

function updateBoard() {
	/* Do nothing if there already is an update running */
	if (g_updating === true)
		return;

	// Lock updating
	g_updating = true;

	var lastId = $("#board tr:last").attr("id").replace("post-", "");
	$.ajax({
		url     : "/backend/since/" + lastId,
		dataType: "xml"
	}).done(function(p_response) {
		var newPostsHtml = "";
		var posts;

		/* Get the reference post. This is one with the lastId.
		 * If there is none, read from the last post.
		 */
		var refPost = $(p_response).find("post[id='" + lastId + "']");
		if (refPost.length > 0) {
			refPost.prevAll().each(function() {
				newPostsHtml += postToHtml($(this));
			});
		}
		else {
			$(p_response).find("post").each(function() {
				newPostsHtml = postToHtml($(this)) + newPostsHtml;
			});
		}

		/* If there are posts to add, append them to the board DOM
		 * then reset the form. */
		if (newPostsHtml.length > 0) {
			$("#board table").append(newPostsHtml);
			$("#message").val("");
			$("#message").focus();
		}
	}).fail(function(p_response, p_status, p_message) {
		console.log("Backend retrieval failed");
		console.log("  message: " + p_message);
		console.log("   status: " + p_status);
	}).always(function() {
		// Release the update lock
		g_updating = false;
	});
}
