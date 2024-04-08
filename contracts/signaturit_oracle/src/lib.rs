#![no_std]

// https://docs.signaturit.com/api/latest#signatures_get_signature
//
// Possibles status
//
// The available status are:
// -- in_queue
//     The document is being processed.
// -- ready
//     The document is ready to be signed.
// -- signing
//     The document is being digitally signed.
// -- completed
//     The document has been signed.
// -- expired
//     The document has expired.
// -- canceled
//     The document has been canceled.
// -- declined
//     The document has been declined.
// -- error
//     There was some error processing the request.
//
// The oracle should be trigger when `completed`, `expired` or `canceled` status
// are meet. Of course, this can be done with a cronjob/backend work to call the
// oracle for us
