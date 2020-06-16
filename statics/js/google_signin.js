var googleUser = {};
var startApp = function() {
  gapi.load('auth2', function(){
    // Retrieve the singleton for the GoogleAuth library and set up the client.
    auth2 = gapi.auth2.init({
      client_id: '732284488135-9cg158hlllk8tubou3ikjrl49m1crosu.apps.googleusercontent.com',
      cookiepolicy: 'single_host_origin',
      // Request scopes in addition to 'profile' and 'email'
      //scope: 'additional_scope'
    });
    attachSignin(document.getElementById('customBtn'));
  });
};

function attachSignin(element) {
  console.log(element.id);
  auth2.attachClickHandler(element, {},
      function(googleUser) {
        var xhr = new XMLHttpRequest();
        var url = "/{{post_url}}/guest_signin";
        xhr.open("POST", url, true);
        xhr.setRequestHeader("Content-Type", "application/json");
        document.getElementById('name').innerText = "Signed in: " +
            googleUser.getBasicProfile().getName();
            console.log(JSON.stringify('guestname: ' + googleUser.getBasicProfile().getName()));
        var guest = {guestname: googleUser.getBasicProfile().getName()};
        xhr.send(JSON.stringify(guest));
        setTimeout(function(){ location.reload(true);}, 3000);
      }, function(error) {
        alert(JSON.stringify(error, undefined, 2));
      });
}