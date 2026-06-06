#include "git_cmd.h"

GitCommand get_git_command(const char *arg) {
    switch (arg[0]) {
    case 'a': {
        switch (arg[1]) {
        case 'd': return kAdd;
        case 'm': return kAm;
        case 'n': return kAnnotate;
        case 'p': return kApply;
        case 'r': {
            switch (arg[2]) {
            case 'c': {
                switch (arg[3]) {
                case 'h': {
                    switch (arg[4]) {
                    case 'i': {
                        switch (arg[5]) {
                        case 'm': return kArchimport;
                        case 'v': return kArchive;
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 't': return kAttributes;
        default: return kNone;
        }
    }
    case 'b': {
        switch (arg[1]) {
        case 'i': return kBisect;
        case 'l': return kBlame;
        case 'r': return kBranch;
        case 'u': {
            switch (arg[2]) {
            case 'g': return kBugreport;
            case 'n': return kBundle;
            default: return kNone;
            }
        }
        default: return kNone;
        }
    }
    case 'c': {
        switch (arg[1]) {
        case 'a': return kCatFile;
        case 'h': {
            switch (arg[2]) {
            case 'e': {
                switch (arg[3]) {
                case 'c': {
                    switch (arg[4]) {
                    case 'k': {
                        switch (arg[5]) {
                        case 'o': {
                            switch (arg[6]) {
                            case 'u': {
                                switch (arg[7]) {
                                case 't': {
                                    switch (arg[8]) {
                                    case 0: return kCheckout;
                                    case '-': return kCheckoutIndex;
                                    default: return kNone;
                                    }
                                }
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        case '-': {
                            switch (arg[6]) {
                            case 'a': return kCheckAttr;
                            case 'i': return kCheckIgnore;
                            case 'm': return kCheckMailmap;
                            case 'r': return kCheckRefFormat;
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                case 'r': {
                    switch (arg[4]) {
                    case 'r': {
                        switch (arg[5]) {
                        case 'y': {
                            switch (arg[6]) {
                            case 0: return kCherry;
                            case '-': return kCherryPick;
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'i': return kCitool;
        case 'l': {
            switch (arg[2]) {
            case 'e': return kClean;
            case 'i': return kCli;
            case 'o': return kClone;
            default: return kNone;
            }
        }
        case 'o': {
            switch (arg[2]) {
            case 'l': return kColumn;
            case 'm': {
                switch (arg[3]) {
                case 'm': {
                    switch (arg[4]) {
                    case 'i': {
                        switch (arg[5]) {
                        case 't': {
                            switch (arg[6]) {
                            case 0: return kCommit;
                            case '-': {
                                switch (arg[7]) {
                                case 'g': return kCommitGraph;
                                case 't': return kCommitTree;
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            case 'n': return kConfig;
            case 'u': return kCountObjects;
            default: return kNone;
            }
        }
        case 'r': {
            switch (arg[2]) {
            case 'e': {
                switch (arg[3]) {
                case 'd': {
                    switch (arg[4]) {
                    case 'e': {
                        switch (arg[5]) {
                        case 'n': {
                            switch (arg[6]) {
                            case 't': {
                                switch (arg[7]) {
                                case 'i': {
                                    switch (arg[8]) {
                                    case 'a': {
                                        switch (arg[9]) {
                                        case 'l': {
                                            switch (arg[10]) {
                                            case 0: return kCredential;
                                            case '-': {
                                                switch (arg[11]) {
                                                case 'c':
                                                    return kCredentialCache;
                                                case 's':
                                                    return kCredentialStore;
                                                default: return kNone;
                                                }
                                            }
                                            default: return kNone;
                                            }
                                        }
                                        default: return kNone;
                                        }
                                    }
                                    default: return kNone;
                                    }
                                }
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'v': {
            switch (arg[2]) {
            case 's': {
                switch (arg[3]) {
                case 'e': return kCvsexportcommit;
                case 'i': return kCvsimport;
                case 's': return kCvsserver;
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        default: return kNone;
        }
    }
    case 'd': {
        switch (arg[1]) {
        case 'a': return kDaemon;
        case 'e': return kDescribe;
        case 'i': {
            switch (arg[2]) {
            case 'a': return kDiagnose;
            case 'f': {
                switch (arg[3]) {
                case 'f': {
                    switch (arg[4]) {
                    case 0: return kDiff;
                    case 't': return kDifftool;
                    case '-': {
                        switch (arg[5]) {
                        case 'f': return kDiffFiles;
                        case 'i': return kDiffIndex;
                        case 't': return kDiffTree;
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        default: return kNone;
        }
    }
    case 'f': {
        switch (arg[1]) {
        case 'a': {
            switch (arg[2]) {
            case 's': {
                switch (arg[3]) {
                case 't': {
                    switch (arg[4]) {
                    case '-': {
                        switch (arg[5]) {
                        case 'e': return kFastExport;
                        case 'i': return kFastImport;
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'e': {
            switch (arg[2]) {
            case 't': {
                switch (arg[3]) {
                case 'c': {
                    switch (arg[4]) {
                    case 'h': {
                        switch (arg[5]) {
                        case 0: return kFetch;
                        case '-': return kFetchPack;
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'i': return kFilterBranch;
        case 'm': return kFmtMergeMsg;
        case 'o': {
            switch (arg[2]) {
            case 'r': {
                switch (arg[3]) {
                case 'm': {
                    switch (arg[4]) {
                    case 'a': {
                        switch (arg[5]) {
                        case 't': {
                            switch (arg[6]) {
                            case '-': {
                                switch (arg[7]) {
                                case 'b': return kFormatBundle;
                                case 'c': {
                                    switch (arg[8]) {
                                    case 'h': return kFormatChunk;
                                    case 'o': return kFormatCommitGraph;
                                    default: return kNone;
                                    }
                                }
                                case 'i': return kFormatIndex;
                                case 'p': {
                                    switch (arg[8]) {
                                    case 'a': {
                                        switch (arg[9]) {
                                        case 'c': return kFormatPack;
                                        case 't': return kFormatPatch;
                                        default: return kNone;
                                        }
                                    }
                                    default: return kNone;
                                    }
                                }
                                case 's': return kFormatSignature;
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                case '-': {
                    switch (arg[4]) {
                    case 'e': {
                        switch (arg[5]) {
                        case 'a': {
                            switch (arg[6]) {
                            case 'c': {
                                switch (arg[7]) {
                                case 'h': {
                                    switch (arg[8]) {
                                    case '-': {
                                        switch (arg[9]) {
                                        case 'r': {
                                            switch (arg[10]) {
                                            case 'e': {
                                                switch (arg[11]) {
                                                case 'f': return kForEachRef;
                                                case 'p': return kForEachRepo;
                                                default: return kNone;
                                                }
                                            }
                                            default: return kNone;
                                            }
                                        }
                                        default: return kNone;
                                        }
                                    }
                                    default: return kNone;
                                    }
                                }
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 's': return kFsck;
        default: return kNone;
        }
    }
    case 'g': {
        switch (arg[1]) {
        case 'c': return kGc;
        case 'e': return kGetTarCommitId;
        case 'i': {
            switch (arg[2]) {
            case 't': {
                switch (arg[3]) {
                case 'k': return kGitk;
                case 'w': return kGitweb;
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'r': return kGrep;
        case 'u': return kGui;
        default: return kNone;
        }
    }
    case 'h': {
        switch (arg[1]) {
        case 'a': return kHashObject;
        case 'e': return kHelp;
        case 'o': {
            switch (arg[2]) {
            case 'o': {
                switch (arg[3]) {
                case 'k': {
                    switch (arg[4]) {
                    case 0: return kHook;
                    case 's': return kHooks;
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 't': return kHttpBackend;
        default: return kNone;
        }
    }
    case 'i': {
        switch (arg[1]) {
        case 'g': return kIgnore;
        case 'm': return kImapSend;
        case 'n': {
            switch (arg[2]) {
            case 'd': return kIndexPack;
            case 'i': return kInit;
            case 's': return kInstaweb;
            case 't': return kInterpretTrailers;
            default: return kNone;
            }
        }
        default: return kNone;
        }
    }
    case 'l': {
        switch (arg[1]) {
        case 'o': return kLog;
        case 's': {
            switch (arg[2]) {
            case '-': {
                switch (arg[3]) {
                case 'f': return kLsFiles;
                case 'r': return kLsRemote;
                case 't': return kLsTree;
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        default: return kNone;
        }
    }
    case 'm': {
        switch (arg[1]) {
        case 'a': {
            switch (arg[2]) {
            case 'i': {
                switch (arg[3]) {
                case 'l': {
                    switch (arg[4]) {
                    case 'i': return kMailinfo;
                    case 'm': return kMailmap;
                    case 's': return kMailsplit;
                    default: return kNone;
                    }
                }
                case 'n': return kMaintenance;
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'e': {
            switch (arg[2]) {
            case 'r': {
                switch (arg[3]) {
                case 'g': {
                    switch (arg[4]) {
                    case 'e': {
                        switch (arg[5]) {
                        case 0: return kMerge;
                        case 't': return kMergetool;
                        case '-': {
                            switch (arg[6]) {
                            case 'b': return kMergeBase;
                            case 'f': return kMergeFile;
                            case 'i': return kMergeIndex;
                            case 'o': return kMergeOneFile;
                            case 't': return kMergeTree;
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'k': {
            switch (arg[2]) {
            case 't': {
                switch (arg[3]) {
                case 'a': return kMktag;
                case 'r': return kMktree;
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'o': return kModules;
        case 'u': return kMultiPackIndex;
        case 'v': return kMv;
        default: return kNone;
        }
    }
    case 'n': {
        switch (arg[1]) {
        case 'a': return kNameRev;
        case 'o': return kNotes;
        default: return kNone;
        }
    }
    case 'p': {
        switch (arg[1]) {
        case 'a': {
            switch (arg[2]) {
            case 'c': {
                switch (arg[3]) {
                case 'k': {
                    switch (arg[4]) {
                    case '-': {
                        switch (arg[5]) {
                        case 'o': return kPackObjects;
                        case 'r': {
                            switch (arg[6]) {
                            case 'e': {
                                switch (arg[7]) {
                                case 'd': return kPackRedundant;
                                case 'f': return kPackRefs;
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            case 't': return kPatchId;
            default: return kNone;
            }
        }
        case 'r': {
            switch (arg[2]) {
            case 'o': {
                switch (arg[3]) {
                case 't': {
                    switch (arg[4]) {
                    case 'o': {
                        switch (arg[5]) {
                        case 'c': {
                            switch (arg[6]) {
                            case 'o': {
                                switch (arg[7]) {
                                case 'l': {
                                    switch (arg[8]) {
                                    case '-': {
                                        switch (arg[9]) {
                                        case 'c': {
                                            switch (arg[10]) {
                                            case 'a':
                                                return kProtocolCapabilities;
                                            case 'o': return kProtocolCommon;
                                            default: return kNone;
                                            }
                                        }
                                        case 'h': return kProtocolHttp;
                                        case 'p': return kProtocolPack;
                                        case 'v': return kProtocolV2;
                                        default: return kNone;
                                        }
                                    }
                                    default: return kNone;
                                    }
                                }
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            case 'u': {
                switch (arg[3]) {
                case 'n': {
                    switch (arg[4]) {
                    case 'e': {
                        switch (arg[5]) {
                        case 0: return kPrune;
                        case '-': return kPrunePacked;
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'u': {
            switch (arg[2]) {
            case 'l': return kPull;
            case 's': return kPush;
            default: return kNone;
            }
        }
        case '4': return kP4;
        default: return kNone;
        }
    }
    case 'q': return kQuiltimport;
    case 'r': {
        switch (arg[1]) {
        case 'a': return kRangeDiff;
        case 'e': {
            switch (arg[2]) {
            case 'a': return kReadTree;
            case 'b': return kRebase;
            case 'f': {
                switch (arg[3]) {
                case 'l': return kReflog;
                case 's': return kRefs;
                default: return kNone;
                }
            }
            case 'm': return kRemote;
            case 'p': {
                switch (arg[3]) {
                case 'a': return kRepack;
                case 'l': {
                    switch (arg[4]) {
                    case 'a': {
                        switch (arg[5]) {
                        case 'c': return kReplace;
                        case 'y': return kReplay;
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                case 'o': return kRepositoryLayout;
                default: return kNone;
                }
            }
            case 'q': return kRequestPull;
            case 'r': return kRerere;
            case 's': {
                switch (arg[3]) {
                case 'e': return kReset;
                case 't': return kRestore;
                default: return kNone;
                }
            }
            case 'v': {
                switch (arg[3]) {
                case 'e': return kRevert;
                case 'i': return kRevisions;
                case '-': {
                    switch (arg[4]) {
                    case 'l': return kRevList;
                    case 'p': return kRevParse;
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'm': return kRm;
        default: return kNone;
        }
    }
    case 's': {
        switch (arg[1]) {
        case 'c': return kScalar;
        case 'e': {
            switch (arg[2]) {
            case 'n': {
                switch (arg[3]) {
                case 'd': {
                    switch (arg[4]) {
                    case '-': {
                        switch (arg[5]) {
                        case 'e': return kSendEmail;
                        case 'p': return kSendPack;
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'h': {
            switch (arg[2]) {
            case 'o': {
                switch (arg[3]) {
                case 'r': return kShortlog;
                case 'w': {
                    switch (arg[4]) {
                    case 0: return kShow;
                    case '-': {
                        switch (arg[5]) {
                        case 'b': return kShowBranch;
                        case 'i': return kShowIndex;
                        case 'r': return kShowRef;
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            case '-': {
                switch (arg[3]) {
                case 'i': return kShI18n;
                case 's': return kShSetup;
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'p': return kSparseCheckout;
        case 't': {
            switch (arg[2]) {
            case 'a': {
                switch (arg[3]) {
                case 's': return kStash;
                case 't': return kStatus;
                default: return kNone;
                }
            }
            case 'r': return kStripspace;
            default: return kNone;
            }
        }
        case 'u': return kSubmodule;
        case 'v': return kSvn;
        case 'w': return kSwitch;
        case 'y': return kSymbolicRef;
        default: return kNone;
        }
    }
    case 't': return kTag;
    case 'u': {
        switch (arg[1]) {
        case 'n': {
            switch (arg[2]) {
            case 'p': {
                switch (arg[3]) {
                case 'a': {
                    switch (arg[4]) {
                    case 'c': {
                        switch (arg[5]) {
                        case 'k': {
                            switch (arg[6]) {
                            case '-': {
                                switch (arg[7]) {
                                case 'f': return kUnpackFile;
                                case 'o': return kUnpackObjects;
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        case 'p': {
            switch (arg[2]) {
            case 'd': {
                switch (arg[3]) {
                case 'a': {
                    switch (arg[4]) {
                    case 't': {
                        switch (arg[5]) {
                        case 'e': {
                            switch (arg[6]) {
                            case '-': {
                                switch (arg[7]) {
                                case 'i': return kUpdateIndex;
                                case 'r': return kUpdateRef;
                                case 's': return kUpdateServerInfo;
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        default: return kNone;
        }
    }
    case 'v': {
        switch (arg[1]) {
        case 'a': return kVar;
        case 'e': {
            switch (arg[2]) {
            case 'r': {
                switch (arg[3]) {
                case 'i': {
                    switch (arg[4]) {
                    case 'f': {
                        switch (arg[5]) {
                        case 'y': {
                            switch (arg[6]) {
                            case '-': {
                                switch (arg[7]) {
                                case 'c': return kVerifyCommit;
                                case 'p': return kVerifyPack;
                                case 't': return kVerifyTag;
                                default: return kNone;
                                }
                            }
                            default: return kNone;
                            }
                        }
                        default: return kNone;
                        }
                    }
                    default: return kNone;
                    }
                }
                case 's': return kVersion;
                default: return kNone;
                }
            }
            default: return kNone;
            }
        }
        default: return kNone;
        }
    }
    case 'w': {
        switch (arg[1]) {
        case 'h': return kWhatchanged;
        case 'o': return kWorktree;
        case 'r': return kWriteTree;
        default: return kNone;
        }
    }
    default: return kNone;
    }
    return kNone;
}
